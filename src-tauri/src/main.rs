#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{borrow::Cow, path::PathBuf};

use clap::Parser;
use serde::Deserialize;
use tauri::{
    api::process::{Command, CommandEvent, TerminatedPayload},
    CustomMenuItem, Manager, Menu, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    Window,
};

#[derive(Parser)]
#[clap(about, author, version)]
struct Opt {
    /// Port where the server listens on.
    #[clap(short, long, default_value_t = 6363)]
    port: u16,
    /// Custom location for MemeBox's configuration.
    #[clap(short, long)]
    config: Option<PathBuf>,
    /// Custom media folder location.
    #[clap(short, long)]
    media: Option<PathBuf>,
}

fn main() {
    let opt = Opt::parse();
    let context = tauri::generate_context!();

    let tray = SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("show_all".to_owned(), "Show All"))
            .add_item(CustomMenuItem::new("hide".to_owned(), "Hide"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("quit".to_owned(), "Quit")),
    );

    tauri::Builder::default()
        .menu(Menu::os_default(&context.package_info().name))
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "show_all" => {
                        for (_, window) in app.windows() {
                            window.show().unwrap();
                        }
                    }
                    "hide" => {
                        app.get_window("main").unwrap().hide().unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                }
            }
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move { run_sidecar(window, opt).await });

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LogLine<'a> {
    category_name: &'a str,
    data: (Cow<'a, str>,),
}

async fn run_sidecar(window: Window, opt: Opt) {
    let mut cmd = Command::new_sidecar("server")
        .expect("failed to setup `server` sidecar")
        .args(["--stdout-json=true"])
        .args([format!("--port={}", opt.port)]);

    if let Some(config) = opt.config {
        cmd = cmd.args([format!("--config={}", config.display())]);
    }

    if let Some(media) = opt.media {
        cmd = cmd.args([format!("--media={}", media.display())]);
    }

    let (mut rx, mut _child) = cmd.spawn().expect("failed to spawn packaged node");
    let mut wait_ready = true;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(stdout) => match serde_json::from_str::<LogLine>(&stdout) {
                Ok(line) => {
                    if wait_ready
                        && line.category_name == "Persistence"
                        && line.data.0 == "Data saved!"
                    {
                        wait_ready = false;
                        window
                            .emit("ready", opt.port)
                            .expect("failed to emit `ready` event");
                    }
                }
                Err(e) => {
                    eprintln!("failed parsing log line: {:?}", e);
                    eprintln!("{}", stdout);
                }
            },
            CommandEvent::Stderr(stderr) => {
                eprintln!("{}", stderr);
            }
            CommandEvent::Error(err) => {
                eprintln!("ERROR: {}", err);
            }
            CommandEvent::Terminated(TerminatedPayload { code, signal }) => {
                // for now we just crash the whole application.
                // TODO: better error recovery
                panic!(
                    "memebox sidecar exited unexpectedly (code: {:?}, signal: {:?})",
                    code, signal
                );
            }
            _ => {}
        }
    }
}
