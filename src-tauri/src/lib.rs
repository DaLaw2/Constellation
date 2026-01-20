mod commands;
mod db;
mod error;
mod state;

use state::{AppConfig, AppState};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

            let db_path = app_data_dir.join("constellation.db");

            // Initialize database pool
            let pool = tauri::async_runtime::block_on(async {
                db::init_database(&db_path)
                    .await
                    .expect("Failed to initialize database")
            });

            // Create app config
            let config = AppConfig {
                db_path: db_path.to_string_lossy().to_string(),
            };

            // Create and manage app state
            let app_state = AppState::new(pool, config);
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Tag Group commands
            commands::tag_groups::create_tag_group,
            commands::tag_groups::get_tag_groups,
            commands::tag_groups::update_tag_group,
            commands::tag_groups::delete_tag_group,
            // Tag commands
            commands::tags::create_tag,
            commands::tags::get_tags_by_group,
            commands::tags::get_all_tags,
            commands::tags::update_tag,
            commands::tags::delete_tag,
            commands::tags::get_tag_usage_counts,
            commands::tags::search_tags,
            // Item commands
            commands::items::create_item,
            commands::items::get_item,
            commands::items::get_item_by_path,
            commands::items::update_item,
            commands::items::soft_delete_item,
            commands::items::restore_item,
            commands::items::get_deleted_items,
            commands::items::permanently_delete_item,
            commands::items::add_tag_to_item,
            commands::items::remove_tag_from_item,
            commands::items::get_tags_for_item,
            commands::items::update_item_tags,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
