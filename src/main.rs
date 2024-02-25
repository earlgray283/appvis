use anyhow::{bail, Result};
use config::Config;
use log::{error, info};
use std::process::{Command, Stdio};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
use tokio::sync::mpsc;
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};
use tray_icon::{menu::MenuEvent, TrayIconBuilder, TrayIconEvent};
use trigger::Trigger;
use util::load_icon;

mod config;
mod trigger;
mod util;

#[tokio::main()]
async fn main() -> Result<()> {
    env_logger::init();

    let config = match Config::load() {
        Ok(c) => match c {
            Some(c) => c,
            None => bail!("config file is not found"),
        },
        Err(e) => bail!("failed to load config: {}", e),
    };

    let mut handles = Vec::new();
    for (name, app) in config.applications {
        info!("[{}] trigger observer is spawned", &name);
        let handle = tokio::spawn(async move {
            let (tx, mut rx) = mpsc::channel(app.trigger.channel_buffer_size());
            tokio::spawn(async move {
                app.trigger.observe(tx).await;
            });
            if let Err(e) = rx.recv().await.unwrap() {
                error!("{}: {}", &name, e);
            }
            info!("[{}] got a result from trigger observer", &name);

            if let Err(e) = Command::new(&app.bin_path)
                .stdin(Stdio::null())
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .spawn()
            {
                error!("{}: {}", &name, e);
            }
        });
        handles.push(handle);
    }

    let mut event_loop = EventLoopBuilder::new().build();
    event_loop.set_activation_policy(ActivationPolicy::Prohibited);
    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu.append_items(&[&PredefinedMenuItem::separator(), &quit_item])?;

    // icon is not displayed if the variable is not assigned
    let _tray_icon = TrayIconBuilder::new()
        .with_icon(load_icon(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/icon.png"
        ))?)
        .with_menu(Box::new(tray_menu))
        .build()?;

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_item.id() {
                *control_flow = ControlFlow::Exit;
            }
            info!("{:?}", event);
        }

        if let Ok(event) = tray_channel.try_recv() {
            info!("{:?}", event);
        }
    })
}
