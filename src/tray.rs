use std::process::Command;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};

pub fn run_tray() -> anyhow::Result<()> {
    let event_loop = EventLoopBuilder::new().build();

    let tray_menu = Menu::new();
    let status_item = MenuItem::new("Status: Running", false, None);
    let upgrade_item = MenuItem::new("Upgrade", true, None);
    let exit_item = MenuItem::new("Exit", true, None);

    tray_menu.append(&status_item)?;
    tray_menu.append(&upgrade_item)?;
    tray_menu.append(&exit_item)?;

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("uncver-artifacts")
        // Note: For now we don't have a path to an icon file,
        // in a real app we'd load an .ico or .png
        .build()?;

    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == upgrade_item.id() {
                println!("Upgrading uncver-artifacts...");
                let _ = Command::new("uncver-artifacts").arg("upgrade").spawn();
            } else if event.id == exit_item.id() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
