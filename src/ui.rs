use bevy::prelude::*;

pub mod buttons;
pub mod launcher_ui;

pub fn run() {
    let app = App::new().add_plugin(launcher_ui::LauncherUI).run();
}
