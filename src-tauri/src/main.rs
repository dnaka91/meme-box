#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{borrow::Cow, path::PathBuf};

use clap::Parser;
use serde::Deserialize;
use tauri::{
    api::process::{Command, CommandEvent, TerminatedPayload},
    Manager, Menu, MenuEntry, MenuItem, Submenu, Window,
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

    tauri::Builder::default()
        .menu(Menu::with_items([
            MenuEntry::Submenu(Submenu::new(
                "memebox",
                Menu::with_items([
                    MenuItem::About("memebox".to_owned()).into(),
                    MenuItem::Separator.into(),
                    MenuItem::Services.into(),
                    MenuItem::Separator.into(),
                    MenuItem::Hide.into(),
                    MenuItem::HideOthers.into(),
                    MenuItem::ShowAll.into(),
                    MenuItem::Separator.into(),
                    MenuItem::Quit.into(),
                ]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "File",
                Menu::with_items([MenuItem::CloseWindow.into()]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "View",
                Menu::with_items([MenuItem::EnterFullScreen.into()]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "Window",
                Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
            )),
        ]))
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move { run_sidecar(window, opt).await });

            Ok(())
        })
        .run(tauri::generate_context!())
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

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(stdout) => match serde_json::from_str::<LogLine>(&stdout) {
                Ok(line) => {
                    if line.category_name == "Persistence" && line.data.0 == "Data saved!" {
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
