// #![windows_subsystem = "windows"]
//TODO Add score counter
//TODO Add high score
//TODO Add main menu
//TODO Add pause screen 
//TODO Add material to target

use bevy::prelude::*;
use rand::Rng;

struct TargetsOnScreen(u8);

struct Target;
struct Speed(f32);
struct Gravity(f32);
struct Score(u32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    InGame,
    MainMenu,
}

pub struct Materials {
    target: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        target: materials.add(asset_server.load("target.png").into()),
    })
}

fn spawn_target(
    mut targets_on_screen: ResMut<TargetsOnScreen>,
    mut commands: Commands,
    materials: Res<Materials>,
    windows: Res<Windows>,
) {
    if targets_on_screen.0 <= 0 {
        let mut rng = rand::thread_rng();
        let window = windows.get_primary().unwrap();

        let target_width = window.width() / 8.0;

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
    mut score: ResMut<Score>,
) {
    if btn.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(mouse_pos) = window.cursor_position() {
            for (tf, sprite, entity) in query.iter_mut() {

                
                let distance = Vec2::from(tf.translation).distance(mouse_pos - Vec2::new(window.width() / 2.0, window.height() / 2.0));
                println!("distance: {}", distance);

                if distance <= sprite.size.x / 2.0
                {
                    println!("pressed on right position");
                    commands.entity(entity).despawn();
                    targets_on_screen.0 -= 1;
                    gravity.0 += 0.1;
                    score.0 += 1;
                }
            }
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
        tf.translation.y -= acc.0 * time.delta_seconds() * (window.height() / 1000.0);
    }
}

fn target_reset(
    mut query: Query<(Entity, &mut Transform), With<Target>>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut targets_on_screen: ResMut<TargetsOnScreen>,
    mut gravity: ResMut<Gravity>,
    mut score: ResMut<Score>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, tf) in query.iter_mut() {
        if tf.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            targets_on_screen.0 -= 1;
            gravity.0 = 1.0;
            score.0 = 0;
        }
    }
}

fn target_despawn (
    mut query: Query<Entity, With<Target>>,
    mut commands: Commands,
)
{
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn switch_to_game(
    kb: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>
) {
    if kb.just_pressed(KeyCode::Space) {
        game_state.set(GameState::InGame).unwrap();
    }
}

fn main() {
    App::build()
        //
        // Plugins
        .add_plugins(DefaultPlugins)
        //
        // Resources
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(TargetsOnScreen(0))
        .insert_resource(Gravity(1.0))
        .insert_resource(Score(0))
        //
        // Add state
        .add_state(GameState::MainMenu)
        //
        // Startup systems
        .add_startup_system(setup.system())
        //
        // System Sets
        //
        // Main Menu set
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
            .with_system(switch_to_game.system())
            
        )
        // InGame set
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(spawn_target.system())
                .with_system(target_movement.system())
                .with_system(target_click.system())
                .with_system(target_reset.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::InGame)
            .with_system(target_despawn.system())
        )
        //
        // Running it
        .run();
}
