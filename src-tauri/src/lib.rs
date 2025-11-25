use tauri::{
    menu::{MenuBuilder, MenuItem, MenuItemBuilder},
    Manager, Url,
};

#[tauri::command]
fn navigate_to(app: tauri::AppHandle, url: String) -> Result<(), String> {
    let window = app.get_webview_window("main").ok_or("Window not found")?;
    window
        .navigate(Url::parse(&url).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            // window.open_devtools();

            // Menu
            // id, name, url

            let home_url = if cfg!(debug_assertions) {
                // dev mode
                "http://localhost:3000"
            } else {
                // production build
                "http://tauri.localhost/"
            };

            struct MenuItemData {
                id: &'static str,
                label: &'static str,
                url: &'static str,
            }
            let menu_items = vec![
                MenuItemData {
                    id: "home",
                    label: "Home",
                    url: home_url,
                },
                MenuItemData {
                    id: "youtube",
                    label: "YouTube",
                    url: "https://www.youtube.com/",
                },
                MenuItemData {
                    id: "chatgpt",
                    label: "ChatGPT",
                    url: "https://chat.openai.com/",
                },
                MenuItemData {
                    id: "gemini",
                    label: "Gemini",
                    url: "https://gemini.google.com/",
                },
                MenuItemData {
                    id: "claude",
                    label: "Claude",
                    url: "https://claude.ai/",
                },
                MenuItemData {
                    id: "grok",
                    label: "Grok",
                    url: "https://grok.com/",
                },
                MenuItemData {
                    id: "deepseek",
                    label: "DeepSeek",
                    url: "https://chat.deepseek.com/",
                },
                // MenuItemData { id: "canva", label: "Canva", url: "https://www.canva.com/" }
            ];

            let menu_items_build: Vec<MenuItem<_>> = menu_items
                .iter()
                .map(|item| {
                    MenuItemBuilder::new(item.label)
                        .id(item.id)
                        .build(app)
                        .unwrap()
                })
                .collect();

            let menu_items_refs: Vec<&dyn tauri::menu::IsMenuItem<_>> = menu_items_build
                .iter()
                .map(|item| item as &dyn tauri::menu::IsMenuItem<_>)
                .collect();

            let menu = MenuBuilder::new(app).items(&menu_items_refs).build()?;

            app.set_menu(menu.clone())?;

            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                println!("Menu Event: {:?}", event.id());
                println!("App Handle: {:?}", app_handle.package_info().name);
                println!("---------------------------");
                println!("Navigating...");
                let w = app_handle.get_webview_window("main").unwrap();
                println!("URL: {:?}", w.url().unwrap());

                let window = app_handle.get_webview_window("main").unwrap();

                // Dynamic menu item handling
                let menu_item = menu_items
                    .iter()
                    .find(|item| item.id == event.id().0.as_str());
                if let Some(item) = menu_item {
                    let _ = window.navigate(Url::parse(item.url).expect("valid url"));
                    return;
                }
            });

            // get url when navigation
            let _ = window.url().map(|url| {
                println!("Navigated to: {}", url);
            })?;

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // app.hide_menu()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![navigate_to])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
