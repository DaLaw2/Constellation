# System Tray Implementation (Tauri 2.x)

## Basic Setup

```rust
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};

fn setup_system_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Create menu items
    let show = MenuItemBuilder::with_id("show", "Show Window").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

    // Build tray icon
    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
```

## JavaScript API

```typescript
import { TrayIcon } from '@tauri-apps/api/tray'
import { Menu } from '@tauri-apps/api/menu'

const menu = await Menu.new({
    items: [
        { id: 'show', text: 'Show Window' },
        { id: 'quit', text: 'Quit' },
    ],
})

const tray = await TrayIcon.new({
    menu,
    menuOnLeftClick: true,
    action: (event) => {
        if (event.type === 'Click') {
            console.log(`Clicked: ${event.button}`)
        }
    },
})
```

## Required Dependencies

```toml
# Cargo.toml
[dependencies]
tauri-plugin-single-instance = "2"  # Prevent multiple instances
tauri-plugin-autostart = "2"        # Optional: run at startup
```

## References

- [Tauri 2 System Tray Guide](https://v2.tauri.app/learn/system-tray/)
- [TrayIcon API Reference](https://v2.tauri.app/reference/javascript/api/namespacetray)
