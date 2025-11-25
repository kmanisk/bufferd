use tauri::{
    menu::{Menu, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        // Setup menu and tray
        .setup(|app| {
            // ---------------- Menu Bar ----------------
            // File
            let file_open = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let file_exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            let file_menu = Submenu::with_items(app, "File", true, &[&file_open, &file_exit])?;

            // Edit
            let edit_cut = MenuItem::with_id(app, "cut", "Cut", true, None::<&str>)?;
            let edit_copy = MenuItem::with_id(app, "copy", "Copy", true, None::<&str>)?;
            let edit_paste = MenuItem::with_id(app, "paste", "Paste", true, None::<&str>)?;
            let edit_menu =
                Submenu::with_items(app, "Edit", true, &[&edit_cut, &edit_copy, &edit_paste])?;

            // Help
            let help_docs = MenuItem::with_id(app, "docs", "Documentation", true, None::<&str>)?;
            let help_issue = MenuItem::with_id(app, "issue", "Report Issue", true, None::<&str>)?;
            let help_menu = Submenu::with_items(app, "Help", true, &[&help_docs, &help_issue])?;

            // About
            let about_item = MenuItem::with_id(app, "about", "About This App", true, None::<&str>)?;
            let about_menu = Submenu::with_items(app, "About", true, &[&about_item])?;

            // Clipboard
            let clip_read =
                MenuItem::with_id(app, "clip_read", "Show Clipboard", true, None::<&str>)?;
            let clip_menu = Submenu::with_items(app, "Clipboard", true, &[&clip_read])?;

            // Root menu bar
            let window_menu = Menu::with_items(
                app,
                &[&file_menu, &edit_menu, &help_menu, &about_menu, &clip_menu],
            )?;
            app.set_menu(window_menu)?;

            // Handle menu events
            app.on_menu_event(|app, event| {
                match event.id.0.as_str() {
                    // File
                    "open" => println!("Open clicked"),
                    "exit" => app.exit(0),

                    // Edit
                    "cut" => println!("Cut clicked"),
                    "copy" => println!("Copy clicked"),
                    "paste" => println!("Paste clicked"),

                    // Help
                    "docs" => println!("Open docs"),
                    "issue" => println!("Open issue tracker"),

                    // About
                    "about" => {
                        println!("About clicked!");
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.eval("alert('This is the About window!')");
                        }
                    }

                    // Clipboard
                    "clip_read" => {
                        let content = app
                            .clipboard()
                            .read_text()
                            .unwrap_or_else(|_| "Clipboard empty".into());
                        println!("Current Clipboard: {}", content);
                        if let Some(win) = app.get_webview_window("main") {
                            let js = format!("alert('Clipboard: {}')", content.replace("'", "\\'"));
                            let _ = win.eval(&js);
                        }
                    }

                    _ => {}
                }
            });

            // ---------------- System Tray ----------------
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                // Handle right-click menu events
                .on_menu_event(|app, event| match event.id.0.as_str() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                        }
                    }
                    "hide" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.hide();
                        }
                    }
                    _ => {}
                })
                // Handle left-click on tray icon
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.unminimize();
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
