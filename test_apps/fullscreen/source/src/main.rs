//! An empty application with default plugins.

use bevy::{app::AppExit, prelude::*, window::WindowMode};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(exit_on_any_input)
        .run();
}

fn exit_on_any_input(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                info!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                info!("{:?} Disconnected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button_type, value)) => {
                match button_type {
                    _ => {
                        app_exit_events.send_default();
                    }
                };
            }

            _ => {}
        }
    }
}
