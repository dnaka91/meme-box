#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api::process::{Command, CommandEvent, TerminatedPayload},
    Manager, Window,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move { run_sidecar(window).await });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn run_sidecar(window: Window) {
    let (mut rx, mut _child) = Command::new_sidecar("memebox")
        .expect("failed to setup `memebox` sidecar")
        .spawn()
        .expect("failed to spawn packaged node");

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(stdout) => {
                println!("{}", stdout);

                // TODO: implement a better readyness detection
                if stdout.contains("Data saved!") {
                    window
                        .emit("ready", ())
                        .expect("failed to emit `ready` event");
                }
            }
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
