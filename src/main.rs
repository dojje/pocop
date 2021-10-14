// #![windows_subsystem = "windows"]
//TODO Add score counter on screen
//TODO Add high score
//DONE Add main menu
//DONE Add pause screen§
//DONE Add material to target
//DONE Add title
//DONE Add start button
//DONE Make the start button start the game
//TODO Mouse should be a crosshair

mod main_menu;
mod pause;
mod ingame;

use bevy::{prelude::*, window::WindowMode};

use main_menu::MainMenuPlugin;
use pause::PausePlugin;
use ingame::InGamePlugin;

use main_menu::MainMenuPlugin;
use pause::PausePlugin;
use ingame::InGamePlugin;

struct Target;
struct Title;
struct PausedScreenRelated;
struct FullscreenButton;
struct MainScreenRelated;
struct StartBtn;
struct Speed(f32);
struct Gravity(f32);
struct Score(u32);
struct FullscreenEnabled(bool);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    InGame,
    MainMenu,
    Paused,
}

pub struct Materials {
    target: Handle<ColorMaterial>,
    title: Handle<ColorMaterial>,
    start_btn: Handle<ColorMaterial>,
    paused_title: Handle<ColorMaterial>,
    fullscreen_text: Handle<ColorMaterial>,
    button: Handle<ColorMaterial>,
    button_pressed: Handle<ColorMaterial>,
}



fn setup(
    mut commands: Commands,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    
    window.set_mode(WindowMode::BorderlessFullscreen);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(Materials {
        target: color_material.add(asset_server.load("target.png").into()),
        title: color_material.add(asset_server.load("pocop.png").into()),
        start_btn: color_material.add(asset_server.load("start_btn.png").into()),
        paused_title: color_material.add(asset_server.load("pause.png").into()),
        fullscreen_text: color_material.add(asset_server.load("fullscreen.png").into()),
        button: color_material.add(asset_server.load("button.png").into()),
        button_pressed: color_material.add(asset_server.load("button_pressed.png").into()),
    })
}

// click on target with mouse
// cotwem








fn main() {
    App::build()
        //
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(InGamePlugin)
        //
        // Resources
        .insert_resource(ClearColor(Color::rgb(0.927, 0.927, 0.927)))
        .insert_resource(Gravity(1.0))
        .insert_resource(Score(0))
        .insert_resource(FullscreenEnabled(true))
        //
        // Add state
        .add_state(GameState::MainMenu)
        //
        // Startup systems
        .add_startup_system(setup.system())
        // InGame set
        //
        // Running it
        .run();
}
