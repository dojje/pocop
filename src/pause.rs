use std::{fs::File, io::Write};

use bevy::{app::AppExit, prelude::*, window::WindowMode};

use crate::{FullscreenEnabled, GameState, Materials, get_config};

struct PausedScreenRelated;
struct ExitGameButton;
struct FullscreenButton;
struct FullscreenEvent(bool);

// Plugin
pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(pause_handler.system())
            .add_system(set_fullscreen_config.system())

            .add_system_set(
                SystemSet::on_enter(GameState::Paused).with_system(paused_setup.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(paused_exit.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                .with_system(fullscreen_listener.system())
                .with_system(exit_listener.system()),
            )
            .add_event::<FullscreenEvent>();
    }
}

fn pause_handler(kb: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if kb.just_pressed(KeyCode::Escape) {
        match game_state.current() {
            GameState::Paused => {
                game_state.pop().unwrap();
            }
            _ => {
                game_state.push(GameState::Paused).unwrap();
            }
        }
    }
}

fn paused_setup(
    mut commands: Commands,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    ui_materials: Res<Materials>,
    fullscreen_enabled: Res<FullscreenEnabled>,
) {
    let check_material = if fullscreen_enabled.0 {
        ui_materials.button_pressed.clone()
    } else {
        ui_materials.button.clone()
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                flex_direction: FlexDirection::ColumnReverse,

                ..Default::default()
            },
            material: color_material.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Title image
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(800.0), Val::Auto),
                        margin: Rect {
                            left: Val::Px(25.0),
                            right: Val::Px(25.0),
                            top: Val::Px(50.0),
                            bottom: Val::Undefined,
                        },
                        max_size: Size::new(Val::Percent(100.0), Val::Auto),
                        min_size: Size::new(Val::Px(190.0), Val::Px(80.0)),

                        ..Default::default()
                    },
                    material: ui_materials.paused_title.clone(),
                    ..Default::default()
                })
                .insert(PausedScreenRelated);
            // holder for fullscreen
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(80.0)),
                        margin: Rect {
                            left: Val::Undefined,
                            right: Val::Undefined,
                            top: Val::Undefined,
                            bottom: Val::Undefined,
                        },
                        max_size: Size::new(Val::Percent(100.0), Val::Auto),

                        ..Default::default()
                    },
                    material: color_material.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Fullscreen thing
                    parent
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(80.0)),
                                margin: Rect {
                                    left: Val::Px(25.0),
                                    right: Val::Px(25.0),
                                    top: Val::Px(50.0),
                                    bottom: Val::Undefined,
                                },
                                max_size: Size::new(Val::Percent(100.0), Val::Auto),

                                ..Default::default()
                            },
                            material: ui_materials.fullscreen_text.clone(),
                            ..Default::default()
                        })
                        .insert(PausedScreenRelated);

                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                                margin: Rect {
                                    left: Val::Px(25.0),
                                    right: Val::Px(25.0),
                                    top: Val::Px(50.0),
                                    bottom: Val::Undefined,
                                },
                                max_size: Size::new(Val::Percent(100.0), Val::Auto),

                                ..Default::default()
                            },
                            material: check_material,
                            ..Default::default()
                        })
                        .insert(FullscreenButton)
                        .insert(PausedScreenRelated);
                })
                .insert(PausedScreenRelated);
                
                let exit_button_height = 162.0;

                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            // size: Size::new(Val::Px(exit_button_height * (28.0 / 16.0)), Val::Px(exit_button_height)),
                            margin: Rect {
                                left: Val::Undefined,
                                right: Val::Undefined,
                                top: Val::Px(100.0),
                                bottom: Val::Px(100.0),
                            },
                            min_size: Size::new(Val::Px(exit_button_height * (28.0 / 16.0)), Val::Px(exit_button_height)),
                            max_size: Size::new(Val::Percent(100.0 * (16.0 / 28.0)), Val::Percent(100.0)),

                            ..Default::default()
                        },
                        material: ui_materials.exit.clone(),
                        ..Default::default()
                    })
                    .insert(ExitGameButton)
                    .insert(PausedScreenRelated);
            
        })
        .insert(PausedScreenRelated);
}

fn paused_exit(mut commands: Commands, mut query: Query<Entity, With<PausedScreenRelated>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn set_fullscreen_config(mut fullscreen_event: EventReader<FullscreenEvent>,) {
    for ev in fullscreen_event.iter() {
        let mut cur_config = get_config();

        cur_config.fullscreen = ev.0;

        let j = serde_json::to_string(&cur_config).unwrap();

        let mut file = File::create("config.json").unwrap();

        file.write(j.as_bytes()).unwrap();
    }
}

fn fullscreen_listener(
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
    ui_materials: Res<Materials>,
    mut windows: ResMut<Windows>,
    mut fullscreen_enabled: ResMut<FullscreenEnabled>,
    mut ev_score: EventWriter<FullscreenEvent>,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let window = windows.get_primary_mut().unwrap();
                if fullscreen_enabled.0 {
                    *material = ui_materials.button.clone();
                    fullscreen_enabled.0 = false;

                    window.set_mode(WindowMode::Windowed);
                } else {
                    *material = ui_materials.button_pressed.clone();
                    fullscreen_enabled.0 = true;
                    window.set_mode(WindowMode::BorderlessFullscreen);
                }
                ev_score.send(FullscreenEvent(fullscreen_enabled.0));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn exit_listener(
    mut query: Query<
        &Interaction,
        (Changed<Interaction>, With<ExitGameButton>),
    >,
    mut exit: EventWriter<AppExit>
) {
    for interaction in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                exit.send(AppExit)
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
