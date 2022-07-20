use bevy::{
    input::gamepad::{GamepadEvent, GamepadEventType},
    prelude::*,
};

pub struct RustArcadePlugin;
impl Plugin for RustArcadePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArcadeInputEvent>()
            .add_system(input_events_system);
    }
}

// Inputs on the arcade machine
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum ArcadeInput {
    JoyUp,
    JoyDown,
    JoyLeft,
    JoyRight,
    JoyButton,
    ButtonTop1,
    ButtonTop2,
    ButtonTop3,
    ButtonTop4,
    ButtonTop5,
    ButtonTop6,
    ButtonLeftSide,
    ButtonRightSide,
    ButtonFront1,
    ButtonFront2,
}

// Event for sending the input data
pub struct ArcadeInputEvent {
    pub gamepad: Gamepad,
    pub arcade_input: ArcadeInput,
    pub value: f32,
}

// Read gamepad inputs and convert to arcade inputs
fn input_events_system(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut arcade_gamepad_event: EventWriter<ArcadeInputEvent>,
) {
    for event in gamepad_event.iter() {
        let arcade_input = match &event.event_type {
            GamepadEventType::Connected => {
                info!("{:?} Connected", event.gamepad);
                None
            }
            GamepadEventType::Disconnected => {
                info!("{:?} Disconnected", event.gamepad);
                None
            }
            GamepadEventType::ButtonChanged(button_type, value) => match button_type {
                GamepadButtonType::DPadUp => Some((ArcadeInput::JoyUp, value)),
                GamepadButtonType::DPadDown => Some((ArcadeInput::JoyDown, value)),
                GamepadButtonType::DPadLeft => Some((ArcadeInput::JoyLeft, value)),
                GamepadButtonType::DPadRight => Some((ArcadeInput::JoyRight, value)),
                GamepadButtonType::South => Some((ArcadeInput::JoyButton, value)),
                GamepadButtonType::East => Some((ArcadeInput::ButtonTop1, value)),
                GamepadButtonType::West => Some((ArcadeInput::ButtonTop2, value)),
                GamepadButtonType::LeftThumb => Some((ArcadeInput::ButtonTop3, value)),
                GamepadButtonType::North => Some((ArcadeInput::ButtonTop4, value)),
                GamepadButtonType::LeftTrigger => Some((ArcadeInput::ButtonTop5, value)),
                GamepadButtonType::RightTrigger => Some((ArcadeInput::ButtonTop6, value)),
                GamepadButtonType::LeftTrigger2 => Some((ArcadeInput::ButtonLeftSide, value)),
                GamepadButtonType::RightTrigger2 => Some((ArcadeInput::ButtonRightSide, value)),
                GamepadButtonType::Select => Some((ArcadeInput::ButtonFront1, value)),
                GamepadButtonType::Start => Some((ArcadeInput::ButtonFront2, value)),
                _ => None,
            },
            GamepadEventType::AxisChanged(_, _) => None,
        };

        if let Some(arcade_input) = arcade_input {
            arcade_gamepad_event.send(ArcadeInputEvent {
                gamepad: event.gamepad,
                arcade_input: arcade_input.0,
                value: *arcade_input.1,
            });
        }
    }
}
