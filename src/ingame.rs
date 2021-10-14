use bevy::{prelude::*};
use rand::Rng;

use crate::{GameState, Gravity, Materials, Score, Speed, Target};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
            .with_system(game_startup.system())
        )
        .add_system_set(
            SystemSet::on_resume(GameState::InGame)
                .with_system(game_startup.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(target_movement.system())
                .with_system(target_click.system())
                .with_system(target_reset.system()),
        )
        .add_system_set(
            SystemSet::on_pause(GameState::InGame)
                .with_system(target_despawn.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::InGame)
            .with_system(target_despawn.system())
        );
    }
}

fn game_startup(
    mut commands: Commands,
    materials: Res<Materials>,
    windows: Res<Windows>,
) {
    spawn_target(&windows, &mut commands, &materials);
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

                    gravity.0 += 0.1;
                    score.0 += 1;
                }
            }
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
) {
    let window = windows.get_primary().unwrap();
    for (entity, tf) in query.iter_mut() {
        if tf.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            gravity.0 = 1.0;
            println!("Score: {}", score.0);
            score.0 = 0;


            spawn_target(&windows, &mut commands, &materials);
        }
    }
}

fn target_despawn(
    mut query: Query<Entity,
    With<Target>>,
    mut commands: Commands,
    mut gravity: ResMut<Gravity>,
    mut score: ResMut<Score>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
        gravity.0 = 1.0;
        score.0 = 0;
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
