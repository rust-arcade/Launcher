use bevy::prelude::*;

use crate::core::launch_app;

use super::{bevy_rust_arcade::ArcadeInputEvent, AppData};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CurrentSelection { index: 0 })
            .add_system(handle_inputs);
    }
}

#[derive(Component)]
pub struct Selectable {
    index: usize,
}

impl Selectable {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn get_index(&self) -> usize {
        self.index
    }
}

pub struct CurrentSelection {
    index: usize,
}

impl CurrentSelection {
    fn previous(&mut self, max_exclusive: usize) {
        if max_exclusive == 0 {
            return;
        }
        self.index = usize::checked_sub(self.index, 1).unwrap_or(max_exclusive - 1) % max_exclusive;
    }
    fn next(&mut self, max_exclusive: usize) {
        if max_exclusive == 0 {
            return;
        }
        self.index = (self.index + 1) % max_exclusive;
    }
    pub fn get(&self) -> usize {
        self.index
    }
}

fn handle_inputs(
    mut selection: ResMut<CurrentSelection>,
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    interaction_query: Query<(&Selectable, &AppData), With<Button>>,
) {
    for event in arcade_input_events.iter() {
        if event.value == 1f32 {
            match &event.arcade_input {
                super::bevy_rust_arcade::ArcadeInput::JoyUp
                | super::bevy_rust_arcade::ArcadeInput::JoyLeft => {
                    selection.previous(interaction_query.iter().count());
                }
                super::bevy_rust_arcade::ArcadeInput::JoyDown
                | super::bevy_rust_arcade::ArcadeInput::JoyRight => {
                    selection.next(interaction_query.iter().count());
                }
                super::bevy_rust_arcade::ArcadeInput::JoyButton => {
                    for (i, (_, app_data)) in interaction_query.iter().enumerate() {
                        if i == selection.get() {
                            if let Ok(mut child) = launch_app(&app_data.path) {
                                child.wait();
                            };
                        }
                    }
                }
                super::bevy_rust_arcade::ArcadeInput::ButtonTop1 => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonTop2 => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonTop3 => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonTop4 => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonTop5 => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonTop6 => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonLeftSide => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonRightSide => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonFront1 => {}
                super::bevy_rust_arcade::ArcadeInput::ButtonFront2 => {}
            }
        }
    }
}
