use bevy::{prelude::*, window::WindowMode};

use crate::{FullscreenButton, FullscreenEnabled, GameState, Materials, PausedScreenRelated};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(pause_handler.system())
        .add_system_set(
            SystemSet::on_enter(GameState::Paused)
            .with_system(paused_setup.system())

        )
        .add_system_set(
            SystemSet::on_exit(GameState::Paused)
            .with_system(paused_exit.system())

        )
        .add_system_set(
            SystemSet::on_update(GameState::Paused)
            .with_system(fullscreen_listener.system())
                
        );
    }
}

fn pause_handler(
    kb: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
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
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                
                ..Default::default()
            },
            material: color_material.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Title image
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(800.0), Val::Auto),
                    margin: Rect { left: Val::Px(25.0), right: Val::Px(25.0), top: Val::Px(50.0), bottom: Val::Undefined },
                    max_size: Size::new(Val::Percent(100.0), Val::Auto),
                    min_size: Size::new(Val::Px(190.0), Val::Px(80.0)),

                    ..Default::default()
                },
                material: ui_materials.paused_title.clone(),
                ..Default::default()
            })
            .insert(PausedScreenRelated);
            // holder for fullscreen
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(80.0)),
                    margin: Rect { left: Val::Px(25.0), right: Val::Px(25.0), top: Val::Px(200.0), bottom: Val::Undefined },
                    max_size: Size::new(Val::Percent(100.0), Val::Auto),

                    ..Default::default()
                },
                material: color_material.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                // Fullscreen thing
                parent.spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(80.0)),
                        margin: Rect { left: Val::Px(25.0), right: Val::Px(25.0), top: Val::Px(50.0), bottom: Val::Undefined },
                        max_size: Size::new(Val::Percent(100.0), Val::Auto),

                        ..Default::default()
                    },
                    material: ui_materials.fullscreen_text.clone(),
                    ..Default::default()
                })
                .insert(PausedScreenRelated);
                
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                        margin: Rect { left: Val::Px(25.0), right: Val::Px(25.0), top: Val::Px(50.0), bottom: Val::Undefined },
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
        }).insert(PausedScreenRelated);
}

fn paused_exit(
    mut commands: Commands,
    mut query: Query<Entity, With<PausedScreenRelated>>
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn fullscreen_listener(
    mut query: Query<(&Interaction, &mut Handle<ColorMaterial>),(Changed<Interaction>, With<Button>),>,
    ui_materials: Res<Materials>,
    mut windows: ResMut<Windows>,
    mut checked: ResMut<FullscreenEnabled>,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let window = windows.get_primary_mut().unwrap();
                if checked.0 {
                    *material = ui_materials.button.clone();
                    checked.0 = false;

    
                    window.set_mode(WindowMode::Windowed);
                    
                }
                else {
                    *material = ui_materials.button_pressed.clone();
                    checked.0 = true;
                    window.set_mode(WindowMode::BorderlessFullscreen);
                }

            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }

}
