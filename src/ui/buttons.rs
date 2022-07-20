use super::launcher_ui::inputs::{CurrentSelection, Selectable};
use bevy::prelude::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .add_system(button_system);
    }
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 1.0, 0.25);
pub const HOVERED_BUTTON_MOUSE: Color = Color::rgb(0.25, 0.5, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.55, 0.75, 0.55);

fn button_system(
    selection: Res<CurrentSelection>,
    mut interaction_query: ParamSet<(
        Query<
            (&Interaction, &Selectable, &mut UiColor, &Children),
            (Changed<Interaction>, With<Button>),
        >,
        Query<(&Selectable, &mut UiColor, &Children), (With<Button>)>,
    )>,
    mut text_query: Query<&mut Text>,
) {
    if interaction_query.p0().iter().count() > 0 || selection.is_changed() {
        for (selectable, mut color, children) in &mut interaction_query.p1() {
            let mut text = text_query.get_mut(children[0]).unwrap();
            if selectable.get_index() == selection.get() {
                *color = HOVERED_BUTTON.into();
            } else {
                *color = NORMAL_BUTTON.into();
            }
        }
        for (interaction, selectable, mut color, children) in &mut interaction_query.p0() {
            let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Clicked => {
                    //text.sections[0].value = "Press".to_string();
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    //text.sections[0].value = "Hover".to_string();
                    *color = HOVERED_BUTTON_MOUSE.into();
                }
                Interaction::None => {
                    //text.sections[0].value = "Button".to_string();
                    *color = NORMAL_BUTTON.into();
                    if selectable.get_index() == selection.get() {
                        *color = HOVERED_BUTTON.into();
                    } else {
                        *color = NORMAL_BUTTON.into();
                    }
                }
            }
        }
    }
}
