use tauri::{
    menu::{Menu, MenuItem, Submenu, SubmenuBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::menu::{Menu, MenuItem, Submenu};

            // ----- FILE SUBMENU -----
            let file_open = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let file_exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;

            let file_menu = Submenu::with_items(app, "File", true, &[&file_open, &file_exit])?;

            // ----- EDIT SUBMENU -----
            let edit_cut = MenuItem::with_id(app, "cut", "Cut", true, None::<&str>)?;
            let edit_copy = MenuItem::with_id(app, "copy", "Copy", true, None::<&str>)?;
            let edit_paste = MenuItem::with_id(app, "paste", "Paste", true, None::<&str>)?;

            let edit_menu =
                Submenu::with_items(app, "Edit", true, &[&edit_cut, &edit_copy, &edit_paste])?;

            // ----- HELP SUBMENU -----
            let help_docs = MenuItem::with_id(app, "docs", "Documentation", true, None::<&str>)?;
            let help_issue = MenuItem::with_id(app, "issue", "Report Issue", true, None::<&str>)?;

            let help_menu = Submenu::with_items(app, "Help", true, &[&help_docs, &help_issue])?;

            // ----- ABOUT SUBMENU -----
            let about_item = MenuItem::with_id(app, "about", "About This App", true, None::<&str>)?;

            let about_menu = Submenu::with_items(app, "About", true, &[&about_item])?;

            // ----- ROOT MENU BAR -----
            let window_menu =
                Menu::with_items(app, &[&file_menu, &edit_menu, &help_menu, &about_menu])?;

            app.set_menu(window_menu)?;
            app.on_menu_event(|app, event| {
                match event.id.0.as_str() {
                    "open" => {
                        println!("Open clicked");
                        // Your action here
                    }
                    "exit" => {
                        app.exit(0);
                    }
                    "cut" => println!("Cut clicked"),
                    "copy" => println!("Copy clicked"),
                    "paste" => println!("Paste clicked"),

                    "docs" => {
                        println!("Open docs");
                        // Example: open a webpage
                        // let _ = tauri_plugin_opener::open_url(app, "https://example.com");
                    }

                    "issue" => {
                        println!("Open issue tracker");
                    }

                    "about" => {
                        println!("About clicked!");
                        if let Some(win) = app.get_webview_window("main") {
                            // Example: call JS in frontend
                            let _ = win.eval("alert('This is the About window!')");
                        }
                    }

                    _ => {}
                }
            });

            // let file_open = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            // let file_exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            //
            // // Submenu::with_items(app, title, enabled, items)
            // let file_submenu = Submenu::with_items(
            //     app,
            //     "File",
            //     true, // enabled
            //     &[&file_open, &file_exit],
            // )?;
            // let window_menu = Menu::with_items(app, &[&file_submenu])?;
            // app.set_menu(window_menu)?;
            // SYSTEM TRAY MENU
            //
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

            //
            // TRAY ICON
            //
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone()) // use app icon
                .menu(&menu)
                // handle right-click menu item actions
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
                // handle tray icon LEFT CLICK events
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            println!("Left Clicked");
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
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
