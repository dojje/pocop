// #![windows_subsystem = "windows"]
//TODO Add score counter on screen
//TODO Add high score
//DONE Add main menu
//TODO Add pause screen
//DONE Add material to target
//DONE Add title
//DONE Add start button
//DONE Make the start button start the game
//TODO Mouse should be a crosshair


use bevy::{ecs::query, prelude::*};
use rand::Rng;

struct Target;

struct Title;
struct PausedScreenEntity;
struct StartBtn;
struct Speed(f32);
struct Gravity(f32);
struct Score(u32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    InGame,
    MainMenu,
    Paused,
}

pub struct Materials {
    target: Handle<ColorMaterial>,
    title: Handle<ColorMaterial>,
    start_btn: Handle<ColorMaterial>
}

fn setup(
    mut commands: Commands,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(Materials {
        target: color_material.add(asset_server.load("target.png").into()),
        title: color_material.add(asset_server.load("pocop.png").into()),
        start_btn: color_material.add(asset_server.load("start_btn.png").into()),
    })
}

fn spawn_target(
    mut commands: Commands,
    materials: Res<Materials>,
    windows: Res<Windows>,
) {
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
// click on target with mouse
// cotwem
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

fn target_reset(
    mut query: Query<(Entity, &mut Transform), With<Target>>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut gravity: ResMut<Gravity>,
    mut score: ResMut<Score>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, tf) in query.iter_mut() {
        if tf.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            gravity.0 = 1.0;
            println!("Score: {}", score.0);
            score.0 = 0;
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

fn switch_to_game(
    mut game_state: ResMut<State<GameState>>,
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<StartBtn>)>,
)
{
    for interaction in interaction_query.iter_mut() {

        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::InGame).unwrap();
            }
            _ => (),
        }
    }
}

fn main_menu_setup(
    mut commands: Commands,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    ui_materials: Res<Materials>,
) {

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
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
                material: ui_materials.title.clone(),
                ..Default::default()
            })
            .insert(Title);

            // Start button
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(400.0), Val::Px(400.0 * (16.0 / 38.0))),
                    margin: Rect { left: Val::Undefined, right: Val::Undefined, top: Val::Auto, bottom: Val::Auto },

                    min_size: Size::new(Val::Px(400.0), Val::Px(400.0 * (16.0 / 38.0))),

                    ..Default::default()
                },
                material: ui_materials.start_btn.clone(),
                ..Default::default()
            })
            .insert(StartBtn);
        });
}

fn despawn_title(
    mut query: Query<Entity, With<Title>>,
    mut commands: Commands,
){
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn despawn_start_button(
    mut query: Query<Entity, With<StartBtn>>,
    mut commands: Commands,
){
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn pausing(
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
) {

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
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
                material: ui_materials.title.clone(),
                ..Default::default()
            })
            .insert(PausedScreenEntity);
        }).insert(PausedScreenEntity);
}

fn paused_exit(
    mut commands: Commands,
    mut query: Query<Entity, With<PausedScreenEntity>>
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn main() {
    App::build()
        //
        // Plugins
        .add_plugins(DefaultPlugins)
        //
        // Resources
        .insert_resource(ClearColor(Color::rgb(0.927, 0.927, 0.927)))
        .insert_resource(Gravity(1.0))
        .insert_resource(Score(0))
        //
        // Add state
        .add_state(GameState::MainMenu)
        //
        // Startup systems
        .add_startup_system(setup.system())
        //
        // Pausing
        .add_system(pausing.system())
        //
        // System Sets
        //
        // Main Menu set
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
            .with_system(main_menu_setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
            .with_system(switch_to_game.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
            .with_system(despawn_title.system())
            .with_system(despawn_start_button.system())
        )
        // Paused
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
                
        )
        // InGame set
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
            .with_system(spawn_target.system())
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
            SystemSet::on_resume(GameState::InGame)
                .with_system(spawn_target.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::InGame)
            .with_system(target_despawn.system())
        )
        //
        // Running it
        .run();
}
