use bevy::prelude::*;
use rand::Rng;

use crate::{Crosshair, GameState, Gravity, Materials, Score, ScoreText, Speed, Target};

pub struct InGamePlugin;

struct ScoreEvent(u32);

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Setup systems
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .with_system(game_startup.system())
                    .with_system(hide_cursor.system())
                    .with_system(spawn_crosshair.system())
                    .with_system(spawn_text.system()),
            )
            .add_system_set(
                SystemSet::on_resume(GameState::InGame)
                    .with_system(game_startup.system())
                    .with_system(hide_cursor.system())
                    .with_system(spawn_crosshair.system())
                    .with_system(spawn_text.system()),
            )
            // Update system
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(target_movement.system())
                    .with_system(target_click.system())
                    .with_system(target_reset.system())
                    .with_system(move_crosshair.system())
                    .with_system(update_text.system()),
            )
            // Shutdown systems
            .add_system_set(
                SystemSet::on_pause(GameState::InGame)
                    .with_system(target_despawn.system())
                    .with_system(show_cursor.system())
                    .with_system(despawn_crosshair.system())
                    .with_system(despawn_text.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::InGame)
                    .with_system(target_despawn.system())
                    .with_system(show_cursor.system())
                    .with_system(despawn_crosshair.system())
                    .with_system(despawn_text.system()),
            )
            // Add event
            .add_event::<ScoreEvent>();
            
    }
}

fn game_startup(mut commands: Commands, materials: Res<Materials>, windows: Res<Windows>) {
    spawn_target(&windows, &mut commands, &materials);
}

fn hide_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_visibility(false);
}

fn show_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_visibility(true);
}

fn spawn_crosshair(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.crosshair.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(60.0, 60.0)),
            ..Default::default()
        })
        .insert(Crosshair);
}

fn despawn_crosshair(mut commands: Commands, mut query: Query<Entity, With<Crosshair>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn spawn_text(mut commands: Commands, mut ev_score: EventWriter<ScoreEvent>, score: Res<Score>,) {
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "0",
            TextStyle {
                ..Default::default()
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    })
    .insert(ScoreText);

    ev_score.send(ScoreEvent(score.0));
}

fn despawn_text(
    mut query: Query<Entity, With<ScoreText>>,
    mut commands: Commands,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn move_crosshair(windows: ResMut<Windows>, mut query: Query<&mut Transform, With<Crosshair>>) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_pos) = window.cursor_position() {
        for mut tf in query.iter_mut() {
            tf.translation.x = mouse_pos.x - window.width() / 2.0;
            tf.translation.y = mouse_pos.y - window.height() / 2.0;
        }
    }
}

fn target_movement(
    windows: Res<Windows>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Transform, &mut Speed), With<Target>>,
    time: Res<Time>,
) {
    for (mut tf, mut acc) in query.iter_mut() {
        let window = windows.get_primary().unwrap();
        acc.0 += gravity.0;
        tf.translation.y -= acc.0 * (window.height() / 1000.0) * time.delta_seconds();
    }
}

fn target_click(
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut query: Query<(&Transform, &Sprite, Entity), With<Target>>,
    mut commands: Commands,
    mut gravity: ResMut<Gravity>,
    mut score: ResMut<Score>,
    materials: Res<Materials>,
    mut ev_score: EventWriter<ScoreEvent>,
) {
    if btn.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(mouse_pos) = window.cursor_position() {
            for (tf, sprite, entity) in query.iter_mut() {
                let distance = Vec2::from(tf.translation)
                    .distance(mouse_pos - Vec2::new(window.width() / 2.0, window.height() / 2.0));

                if distance <= sprite.size.x / 2.0 {
                    commands.entity(entity).despawn();
                    let mut rng = rand::thread_rng();
                    let window = windows.get_primary().unwrap();

                    let target_width = (window.width() / 30.0 * window.height() / 30.0) / 8.0;

                    let target_x = rng.gen_range(
                        -window.width() / 2.0 + target_width / 2.0
                            ..window.width() / 2.0 - target_width / 2.0,
                    );
                    let target_y = window.height() / 2.0 - target_width / 2.0;

                    commands
                        .spawn_bundle(SpriteBundle {
                            material: materials.target.clone(),
                            transform: Transform::from_xyz(target_x, target_y, 0.0),
                            sprite: Sprite::new(Vec2::new(target_width, target_width)),
                            ..Default::default()
                        })
                        .insert(Target)
                        .insert(Speed(0.0));

                    gravity.0 += 0.1;
                    score.0 += 1;
                    ev_score.send(ScoreEvent(score.0));
                }
            }
        }
    }
}

fn update_text (
    mut ev_levelup: EventReader<ScoreEvent>,
    mut query: Query<&mut Text, With<ScoreText>>,
    materials: Res<Materials>,
) {
    for score in ev_levelup.iter() {
        println!("{}", score.0);

        for mut text in query.iter_mut()  {
            text.sections = vec![
                TextSection {
                    value: format!("{}", score.0),
                    style: TextStyle {
                        font: materials.font.clone(),
                        font_size: 60.0,
                        color: Color::rgb(0.75, 0.75, 0.75),
                    },
                },
            ];
        }
    }
}

fn target_reset(
    mut query: Query<(Entity, &mut Transform), With<Target>>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut gravity: ResMut<Gravity>,
    mut score: ResMut<Score>,
    materials: Res<Materials>,
    mut ev_score: EventWriter<ScoreEvent>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, tf) in query.iter_mut() {
        if tf.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            gravity.0 = 1.0;
            println!("Score: {}", score.0);
            score.0 = 0;

            ev_score.send(ScoreEvent(score.0));

            spawn_target(&windows, &mut commands, &materials);
        }
    }
}

fn target_despawn(
    mut query: Query<Entity, With<Target>>,
    mut commands: Commands,
    mut gravity: ResMut<Gravity>,
    mut score: ResMut<Score>,
    mut ev_score: EventWriter<ScoreEvent>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
        gravity.0 = 1.0;
        score.0 = 0;

        ev_score.send(ScoreEvent(score.0));
    }
}

fn spawn_target(windows: &Res<Windows>, commands: &mut Commands, materials: &Res<Materials>) {
    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();
    let target_width = (window.width() / 30.0 * window.height() / 30.0) / 8.0;
    let target_x = rng.gen_range(
        -window.width() / 2.0 + target_width / 2.0..window.width() / 2.0 - target_width / 2.0,
    );
    let target_y = window.height() / 2.0 - target_width / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.target.clone(),
            transform: Transform::from_xyz(target_x, target_y, 0.0),
            sprite: Sprite::new(Vec2::new(target_width, target_width)),
            ..Default::default()
        })
        .insert(Target)
        .insert(Speed(0.0));
}
