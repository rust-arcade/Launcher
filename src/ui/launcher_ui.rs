use std::path::PathBuf;

use super::buttons;
use super::buttons::ButtonPlugin;
use bevy::{prelude::*, winit::WinitSettings};

pub struct LauncherUI;

impl Plugin for LauncherUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(ButtonPlugin)
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            .add_startup_system(setup)
            .add_system(button_to_launch);
    }
}

#[derive(Component)]
struct AppData {
    pub path: PathBuf,
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
                parent
                    .spawn_bundle(ButtonBundle {
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
                    })
                    .insert(AppData { path: path.clone() })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                path.file_name().unwrap().to_string_lossy(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..default()
                        });
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
