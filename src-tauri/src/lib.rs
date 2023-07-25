use std::sync::Arc;

use tauri::{App, AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn run() {
    let mut app = tauri::Builder::default()
        .system_tray(build_tray())
        .on_system_tray_event(on_system_tray_event)
        .setup(start_break_flow)
        .build(tauri::generate_context!())
        .expect("failed to build the tauri app");

    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => api.prevent_exit(),
        _ => {}
    });
}

fn build_break_window(app_handle: &AppHandle) -> tauri::Window {
    tauri::WindowBuilder::new(
        app_handle,
        "break",
        tauri::WindowUrl::App("index.html".into()),
    )
    .always_on_top(true)
    .decorations(false)
    .skip_taskbar(true)
    .maximized(true)
    .transparent(true)
    .maximizable(false)
    .minimizable(false)
    .resizable(false)
    .closable(false)
    .focused(false)
    .build()
    .unwrap()
}

fn build_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new().add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    SystemTray::new().with_menu(tray_menu)
}

fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => app.exit(0),
            _ => {}
        },
        _ => {}
    }
}

fn format_time(time: i32) -> String {
    if time < 10 {
        format!("0{}", time)
    } else {
        format!("{}", time)
    }
}

fn start_break_flow(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    let tray_handle = app.tray_handle();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
    let sender = Arc::new(tx);

    tauri::async_runtime::spawn(async move {
        loop {
            tray_handle.set_title("break").unwrap();

            let sender = Arc::clone(&sender);
            let break_window = build_break_window(&app_handle);

            break_window.on_window_event(move |event| match event {
                tauri::WindowEvent::Destroyed => {
                    tauri::async_runtime::block_on(async {
                        let _ = sender.send(()).await;
                    });
                }
                _ => {}
            });

            let _ = rx.recv().await.unwrap();

            let mut work_time = 60 * 50;

            for _ in 0..work_time {
                work_time -= 1;

                let minutes = work_time / 60;
                let seconds = work_time - ((work_time / 60) * 60);

                tray_handle
                    .set_title(
                        format!("{}:{}", format_time(minutes), format_time(seconds)).as_str(),
                    )
                    .unwrap();

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    });

    Ok(())
}
