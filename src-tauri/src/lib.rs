#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri_plugin_sql::{Migration, MigrationKind};

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
mod cmd;

pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_projects_table",
        sql: "CREATE TABLE projects 
            (id INTEGER PRIMARY KEY, 
            name TEXT NOT NULL, 
            description TEXT,
            path TEXT NOT NULL,
            status TEXT NOT NULL,
            tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
        kind: MigrationKind::Up,
    }];
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:ddevui.db", migrations)
                .build(),
        )
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    other => {
                        println!("menu item {} not handled", other);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![cmd::get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
