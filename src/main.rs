// #![windows_subsystem = "windows"]

use bevy::{input::mouse::MouseMotion, math::Vec2Swizzles, prelude::*};


struct TargetsOnScreen(u8);

struct Target;
struct Speed(f32);
struct Gravity(f32);

pub struct Materials {
    _target: Handle<ColorMaterial>,

}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

}

fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
) {
    for ev in cursor_evr.iter() {
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.id
        );
    }
}

fn spawn_target(
    mut targets_on_screen: ResMut<TargetsOnScreen>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if targets_on_screen.0 <= 0 {
        commands
            .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(120.0, 120.0)),
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
    mut query: Query<(&Transform, &Sprite, With<Target>)>
    
) {
    if btn.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(mouse_pos) = window.cursor_position() {

            for (tf, sprite ,_) in query.iter_mut() {
                println!("target x: {}, target y: {}", tf.translation.x + window.width() / 2.0 - sprite.size.x / 2.0, tf.translation.y + window.height() / 2.0 - sprite.size.y / 2.0);
                println!("mouse  x: {}, mouse  y: {}", mouse_pos.x, window.height() - mouse_pos.y);

                let t_x = tf.translation.x + window.width() / 2.0 - sprite.size.x / 2.0;
                let t_y = tf.translation.y + window.height() / 2.0 - sprite.size.y / 2.0;

                let m_x = mouse_pos.x;
                let m_y = window.height() - mouse_pos.y;

                if 
                t_x < m_x &&
                m_x < t_x + sprite.size.x &&

                // You have to do that with the mouse because y: 0 is at bottom
                t_y < m_y &&
                m_y < t_y + sprite.size.y
                
                {
                    println!("pressed on right position");
                }
            }
        }
    }
}

fn target_movement(
    gravity: Res<Gravity>,
    mut query: Query<(&mut Transform, &mut Speed, With<Target>)>
) {
    for (mut tf, mut acc, _) in query.iter_mut() {
        acc.0 += gravity.0;
        tf.translation.y -= acc.0;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)

        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(TargetsOnScreen(0))
        .insert_resource(Gravity(0.04))

        .add_startup_system(setup.system())

        .add_system(spawn_target.system())
        // .add_system(target_movement.system())
        .add_system(target_click.system())
        // .add_system(cursor_events.system())
        
        .run();
}
