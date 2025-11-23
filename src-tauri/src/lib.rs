use tauri::{menu::MenuBuilder, Manager, Url};

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
            let menu = MenuBuilder::new(app)
                .text("home", "Home")
                .text("youtube", "YouTube")
                .text("chatgpt", "ChatGPT")
                .text("gemini", "Gemini")
                .text("claude", "Claude")
                .text("grok", "Grok")
                .text("deepseek", "DeepSeek")
                .build()?;

            app.set_menu(menu.clone())?;

            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                println!("Menu Event: {:?}", event.id());
                println!("App Handle: {:?}", app_handle.package_info().name);
                println!("---------------------------");
                println!("Navigating...");
                let w = app_handle.get_webview_window("main").unwrap();
                println!("URL: {:?}", w.url().unwrap());

                let window = app_handle.get_webview_window("main").unwrap();
                let home_url = if cfg!(debug_assertions) {
                    // dev mode
                    "http://localhost:3000"
                } else {
                    // production build
                    "http://tauri.localhost/"
                };
                // let home_url = "http://tauri.localhost/";

                match event.id().0.as_str() {
                    "home" => {
                        let _ = window.navigate(Url::parse(&home_url).expect("valid url"));
                    }
                    "youtube" => {
                        let _ = window
                            .navigate(Url::parse("https://www.youtube.com/").expect("valid url"));
                    }
                    "chatgpt" => {
                        let _ = window
                            .navigate(Url::parse("https://chat.openai.com/").expect("valid url"));
                    }
                    "gemini" => {
                        let _ = window
                            .navigate(Url::parse("https://gemini.google.com/").expect("valid url"));
                    }
                    "claude" => {
                        let _ =
                            window.navigate(Url::parse("https://claude.ai/").expect("valid url"));
                    }
                    "grok" => {
                        let _ =
                            window.navigate(Url::parse("https://grok.com/").expect("valid url"));
                    }
                    "deepseek" => {
                        let _ = window
                            .navigate(Url::parse("https://chat.deepseek.com/").expect("valid url"));
                    }
                    _ => {}
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
