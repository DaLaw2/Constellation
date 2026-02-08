//! Constellation Application
//!
//! A file tagging system built with Tauri and DDD architecture.

mod application;
mod commands;
mod domain;
mod error;
mod infrastructure;
mod state;

use infrastructure::persistence::init_database;
use state::{AppConfig, AppState};
use tauri::http::Response;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .register_asynchronous_uri_scheme_protocol("thumb", |ctx, request, responder| {
            let app = ctx.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let response = handle_thumb_request(&app, &request).await;
                responder.respond(response);
            });
        })
        .setup(|app| {
            // Initialize database
            // SAFETY: App data directory is essential for application to function.
            // If we cannot get or create it, the application cannot proceed.
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory - this is a critical system error");
            std::fs::create_dir_all(&app_data_dir).expect(
                "Failed to create app data directory - insufficient permissions or disk full",
            );

            let db_path = app_data_dir.join("constellation.db");

            // Initialize database pool
            // SAFETY: Database initialization is critical for application functionality.
            // If database cannot be initialized, the application cannot function.
            let pool = tauri::async_runtime::block_on(async {
                init_database(&db_path)
                    .await
                    .expect("Failed to initialize database - check disk space and permissions")
            });

            // Create app config
            let config = AppConfig {
                db_path: db_path.to_string_lossy().to_string(),
            };

            // Create and manage app state
            let app_state = AppState::new(pool, config, app_data_dir.clone());
            app.manage(app_state);

            // Spawn background cache eviction on startup
            let thumb_service = app.state::<AppState>().thumbnail_service.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = thumb_service.evict_cache().await {
                    eprintln!("Background cache eviction failed: {}", e);
                }
            });

            // Auto-refresh USN index on startup if enabled
            let refresh_service = app.state::<AppState>().usn_refresh_service.clone();
            let settings = app.state::<AppState>().settings_service.clone();

            tauri::async_runtime::spawn(async move {
                let auto_refresh = settings
                    .get("usn_auto_refresh")
                    .await
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| "false".to_string());

                if auto_refresh == "true" {
                    let drives: Vec<char> = ('A'..='Z')
                        .filter(|&c| {
                            crate::infrastructure::usn_journal::is_ntfs(c).unwrap_or(false)
                        })
                        .collect();

                    if let Err(e) = refresh_service.refresh(&drives).await {
                        eprintln!("Auto USN refresh failed: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Tag Group commands
            commands::tag_groups::create_tag_group,
            commands::tag_groups::get_tag_groups,
            commands::tag_groups::update_tag_group,
            commands::tag_groups::delete_tag_group,
            commands::tag_groups::reorder_tag_groups,
            // Tag commands
            commands::tags::create_tag,
            commands::tags::get_tags_by_group,
            commands::tags::get_all_tags,
            commands::tags::update_tag,
            commands::tags::delete_tag,
            commands::tags::get_tag_usage_counts,
            commands::tags::search_tags,
            commands::tags::merge_tags,
            // Item commands
            commands::items::create_item,
            commands::items::get_item,
            commands::items::get_item_by_path,
            commands::items::get_items_by_paths,
            commands::items::update_item,
            commands::items::delete_item,
            commands::items::add_tag_to_item,
            commands::items::remove_tag_from_item,
            commands::items::get_tags_for_item,
            commands::items::get_tags_for_items,
            commands::items::update_item_tags,
            commands::items::batch_add_tag_to_items,
            commands::items::batch_remove_tag_from_items,
            commands::items::get_common_tags_for_paths,
            // Tag Template commands
            commands::tag_templates::create_tag_template,
            commands::tag_templates::get_tag_templates,
            commands::tag_templates::apply_tag_template,
            commands::tag_templates::delete_tag_template,
            commands::tag_templates::update_tag_template,
            // File System commands
            commands::filesystem::get_drives,
            commands::filesystem::read_directory,
            commands::filesystem::get_file_metadata,
            commands::filesystem::open_file_external,
            commands::filesystem::reveal_in_explorer,
            // Search commands
            commands::search::search_items_by_tags_and,
            commands::search::search_items_by_tags_or,
            commands::search::search_items_by_filename,
            commands::search::search_items,
            commands::search::search_cql,
            commands::search::get_recent_search_history,
            commands::search::delete_search_history,
            commands::search::clear_search_history,
            // Settings commands
            commands::settings::get_all_settings,
            commands::settings::update_setting,
            commands::settings::reset_setting,
            // Thumbnail commands
            commands::thumbnails::get_cache_stats,
            commands::thumbnails::clear_thumbnail_cache,
            // File Monitor commands
            commands::file_monitor::refresh_file_index,
            commands::file_monitor::check_usn_support,
            commands::file_monitor::get_usn_drive_status,
            commands::file_monitor::enable_usn_journal,
        ])
        .run(tauri::generate_context!())
        // SAFETY: This is the main entry point. If Tauri runtime fails to start,
        // there is no recovery path - the application cannot run.
        .expect("error while running tauri application");
}

/// Handle `thumb://localhost/{encoded_path}?size={size}` requests.
///
/// URL format: `thumb://localhost/{url_encoded_path}?size={thumb_size}`
/// Returns WebP image bytes with aggressive caching headers.
async fn handle_thumb_request(
    app: &tauri::AppHandle,
    request: &tauri::http::Request<Vec<u8>>,
) -> Response<Vec<u8>> {
    let uri = request.uri().to_string();

    // Parse path and query from URI
    let parsed = match parse_thumb_uri(&uri) {
        Some(p) => p,
        None => return thumb_error_response(400, "Invalid thumbnail URL"),
    };

    // Read file metadata for cache key
    let metadata = match std::fs::metadata(&parsed.path) {
        Ok(m) => m,
        Err(_) => return thumb_error_response(404, "File not found"),
    };

    let mtime = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let file_size = metadata.len();

    // Get thumbnail service from app state
    let state = app.state::<AppState>();

    match state
        .thumbnail_service
        .get_thumbnail(&parsed.path, mtime, file_size, parsed.size)
        .await
    {
        Ok(webp_bytes) => Response::builder()
            .status(200)
            .header("Content-Type", "image/webp")
            .header("Cache-Control", "public, max-age=31536000, immutable")
            .body(webp_bytes)
            .unwrap_or_else(|_| thumb_error_response(500, "Failed to build response")),
        Err(_) => thumb_error_response(404, "Failed to generate thumbnail"),
    }
}

struct ThumbUriParsed {
    path: String,
    size: u32,
}

/// Parse thumb URI into path and size.
/// Handles both `http://thumb.localhost/` (WebView2) and `thumb://localhost/` formats.
fn parse_thumb_uri(uri: &str) -> Option<ThumbUriParsed> {
    let after_scheme = uri
        .strip_prefix("http://thumb.localhost/")
        .or_else(|| uri.strip_prefix("https://thumb.localhost/"))
        .or_else(|| uri.strip_prefix("thumb://localhost/"))?;

    // Split path and query
    let (path_encoded, query) = match after_scheme.find('?') {
        Some(idx) => (&after_scheme[..idx], &after_scheme[idx + 1..]),
        None => (after_scheme, ""),
    };

    // URL decode the path
    let path = percent_decode(path_encoded);
    if path.is_empty() {
        return None;
    }

    // Parse size from query string (default 256)
    let size = parse_query_param(query, "size")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(256);

    Some(ThumbUriParsed { path, size })
}

/// Simple URL percent-decoding.
fn percent_decode(input: &str) -> String {
    let mut result = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&input[i + 1..i + 3], 16) {
                result.push(byte);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).into_owned()
}

/// Extract a query parameter value by key.
fn parse_query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    for pair in query.split('&') {
        if let Some(val) = pair
            .strip_prefix(key)
            .and_then(|rest| rest.strip_prefix('='))
        {
            return Some(val);
        }
    }
    None
}

fn thumb_error_response(status: u16, msg: &str) -> Response<Vec<u8>> {
    Response::builder()
        .status(status)
        .header("Content-Type", "text/plain")
        .body(msg.as_bytes().to_vec())
        .unwrap_or_else(|_| Response::builder().status(500).body(Vec::new()).unwrap())
}
