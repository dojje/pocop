// #![windows_subsystem = "windows"]

use bevy::prelude::*;
use rand::Rng;

struct TargetsOnScreen(u8);

struct Target;
struct Speed(f32);
struct Gravity(f32);

pub struct Materials {
    _target: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_target(
    mut targets_on_screen: ResMut<TargetsOnScreen>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    if targets_on_screen.0 <= 0 {
        let mut rng = rand::thread_rng();
        let window = windows.get_primary().unwrap();

        let target_width = window.width() / 10.0;

        let target_x = rng.gen_range(-window.width() / 2.0 + target_width / 2.0 .. window.width() / 2.0 - target_width / 2.0);
        let target_y = window.height() / 2.0 - target_width / 2.0;

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                transform: Transform::from_xyz(target_x, target_y, 0.0),
                sprite: Sprite::new(Vec2::new(target_width, target_width)),
                ..Default::default()
            })
            .insert(Target)
            .insert(Speed(0.0));

        targets_on_screen.0 += 1;
    }
}

fn target_click(
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut query: Query<(&Transform, &Sprite, Entity), With<Target>>,
    mut commands: Commands,
    mut targets_on_screen: ResMut<TargetsOnScreen>,
    mut gravity: ResMut<Gravity>,
) {
    if btn.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(mouse_pos) = window.cursor_position() {
            for (tf, sprite, entity) in query.iter_mut() {
                println!(
                    "target x: {}, target y: {}",
                    tf.translation.x + window.width() / 2.0 - sprite.size.x / 2.0,
                    tf.translation.y + window.height() / 2.0 - sprite.size.y / 2.0
                );
                println!(
                    "mouse  x: {}, mouse  y: {}",
                    mouse_pos.x,
                    window.height() - mouse_pos.y
                );
                println!(
                    "window w: {}, window h: {}",
                    window.width(),
                    window.height(),
                );

                let t_x = tf.translation.x + window.width() / 2.0 - sprite.size.x / 2.0;
                let t_y = tf.translation.y + window.height() / 2.0 - sprite.size.y / 2.0;

                if t_x < mouse_pos.x &&
                mouse_pos.x < t_x + sprite.size.x &&

                // You have to do that with the mouse because y: 0 is at bottom
                t_y < mouse_pos.y &&
                mouse_pos.y < t_y + sprite.size.y
                {
                    println!("pressed on right position");
                    commands.entity(entity).despawn();
                    targets_on_screen.0 -= 1;
                    gravity.0 += 0.1;

                }
            }
        }
    }
}

fn target_movement(
    gravity: Res<Gravity>,
    mut query: Query<(&mut Transform, &mut Speed), With<Target>>,
    time: Res<Time>,
) {
    for (mut tf, mut acc) in query.iter_mut() {
        acc.0 += gravity.0;
        tf.translation.y -= acc.0 * time.delta_seconds();
    }
}

fn target_reset(
    mut query: Query<(Entity, &mut Transform), With<Target>>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut targets_on_screen: ResMut<TargetsOnScreen>,
    mut gravity: ResMut<Gravity>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, tf) in query.iter_mut() {
        if tf.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            targets_on_screen.0 -= 1;
            gravity.0 = 1.0;
        }
    }


}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(TargetsOnScreen(0))
        .insert_resource(Gravity(1.0))

        .add_startup_system(setup.system())
        .add_system(spawn_target.system())
        .add_system(target_movement.system())
        .add_system(target_click.system())
        .add_system(target_reset.system())

        .run();
}
