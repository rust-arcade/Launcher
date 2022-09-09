use std::{fs, path::PathBuf};

use crate::ui::launcher_ui::inputs::Selectable;

use self::{
    fake_arcade::KeyToArcade,
    inputs::{CurrentSelection, InputPlugin},
};
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
        .add_system(fake_arcade::input_system)
        .add_system(big_image_background)
        .add_system(description_background);
    }
}

#[derive(Component)]
struct AppData {
    pub path: PathBuf,
}
#[derive(Component, Deserialize, Debug)]
struct AppMetaSerialized {
    pub description: String,
    pub image_path: Option<String>,
}
#[derive(Component, Debug)]
struct AppMeta {
    pub description: String,
    pub image: Handle<Image>,
}

#[derive(Component, Debug)]
struct BigPreview;

#[derive(Component, Debug)]
struct Description;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(Camera2dBundle::default());
    let handle_placeholder_big = asset_server.load("placeholder_big.png");

    let paths = crate::core::list_games();
    // Big Preview
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    image: handle_placeholder_big.clone().into(),
                    ..default()
                })
                .insert(BigPreview);
        });
    // Description
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(-200.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_sections([TextSection::new(
                    "description",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )]))
                .insert(Description);
        });

    // Buttons
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
                if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                    let mut file_name = String::from(file_name);
                    file_name.push_str(".meta");
                    let meta_path = path.with_file_name(file_name);
                    if let Ok(contents) = fs::read_to_string(meta_path) {
                        let deserialized: AppMetaSerialized =
                            serde_json::from_str(&contents).unwrap();
                        let image_big_handle = if let Some(big_image_path) = deserialized.image_path
                        {
                            asset_server.load(&format!(
                                "../{}/{}",
                                path.to_str().unwrap(),
                                big_image_path.clone()
                            ))
                        } else {
                            handle_placeholder_big.clone()
                        };
                        launchable.insert(AppMeta {
                            description: deserialized.description,
                            image: image_big_handle,
                        });
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

fn big_image_background(
    selection: Res<CurrentSelection>,
    interaction_query: Query<Option<&AppMeta>, (With<Selectable>, With<AppData>, With<Button>)>,
    mut big_preview: Query<&mut UiImage, With<BigPreview>>,
) {
    if selection.is_changed() {
        for (i, meta) in interaction_query.iter().enumerate() {
            if i == selection.get() {
                if let Some(meta) = meta {
                    let mut handle = big_preview.single_mut();
                    handle.0 = meta.image.clone();
                }
                // TODO: put placeholder image to see the change
            }
        }
    }
}
fn description_background(
    selection: Res<CurrentSelection>,
    interaction_query: Query<Option<&AppMeta>, (With<Selectable>, With<AppData>, With<Button>)>,
    mut description: Query<&mut Text, With<Description>>,
) {
    if selection.is_changed() {
        for (i, meta) in interaction_query.iter().enumerate() {
            if i == selection.get() {
                if let Some(meta) = meta {
                    let mut text = description.single_mut();
                    text.sections[0].value = meta.description.clone();
                }
            }
        }
    }
}
