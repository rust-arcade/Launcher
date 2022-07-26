use std::{fs, path::PathBuf};

use crate::ui::launcher_ui::inputs::Selectable;

use self::{fake_arcade::KeyToArcade, inputs::InputPlugin};
use serde::Deserialize;

use super::buttons;
use super::buttons::ButtonPlugin;
use bevy::{prelude::*, window::WindowMode, winit::WinitSettings};

mod bevy_rust_arcade;
mod fake_arcade;
pub mod inputs;

pub struct LauncherUI;

impl Plugin for LauncherUI {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_rust_arcade::RustArcadePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(ButtonPlugin)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(button_to_launch)
        .insert_resource(KeyToArcade::default())
        .add_system(fake_arcade::input_system);
    }
}

#[derive(Component)]
struct AppData {
    pub path: PathBuf,
}
#[derive(Component, Deserialize, Debug)]
struct AppMeta {
    pub description: String,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(Camera2dBundle::default());

    let paths = crate::core::list_games();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::SpaceBetween,
                // vertically center child text
                align_items: AlignItems::Stretch,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for (i, path) in paths.iter().enumerate() {
                dbg!(i);
                let mut launchable = parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: buttons::NORMAL_BUTTON.into(),
                    ..default()
                });
                launchable.insert(AppData { path: path.clone() });
                dbg!(path.file_name());
                if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                    let mut file_name = String::from(file_name);
                    file_name.push_str(".meta");
                    let meta_path = path.with_file_name(file_name);
                    if let Ok(contents) = fs::read_to_string(meta_path) {
                        let deserialized: AppMeta = serde_json::from_str(&contents).unwrap();
                        dbg!(deserialized);
                    }
                }
                launchable
                    .insert(Selectable::new(i))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_sections([TextSection::new(
                            path.file_name().unwrap().to_string_lossy(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )]));
                    });
            }
        });
}

fn button_to_launch(
    mut interaction_query: Query<(&Interaction, &AppData), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, data) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Ok(mut child) = crate::core::launch_app(&data.path) {
                    child.wait();
                };
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
