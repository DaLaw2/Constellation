fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        panic!("This application is only supported on Windows.");
    }
    tauri_build::build()
}
