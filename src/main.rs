// #![windows_subsystem = "windows"]
//TODO Add score counter on screen
//TODO Add high score
//DONE Add main menu
//DONE Add pause screen§
//DONE Add material to target
//DONE Add title
//DONE Add start button
//DONE Make the start button start the game
//DONE Mouse should be a crosshair

mod ingame;
mod main_menu;
mod pause;

use std::fs;

use bevy::{prelude::*, window::WindowMode};

use serde::{Serialize, Deserialize};

use ingame::InGamePlugin;
use main_menu::MainMenuPlugin;
use pause::PausePlugin;

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
struct Crosshair;
struct ScoreText;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    InGame,
    MainMenu,
    Paused,
}
#[derive(Serialize, Deserialize)]
struct Config {
    fullscreen: bool,
}

pub struct Materials {
    target: Handle<ColorMaterial>,
    title: Handle<ColorMaterial>,
    start_btn: Handle<ColorMaterial>,
    paused_title: Handle<ColorMaterial>,
    fullscreen_text: Handle<ColorMaterial>,
    button: Handle<ColorMaterial>,
    button_pressed: Handle<ColorMaterial>,
    crosshair: Handle<ColorMaterial>,
    font: Handle<Font>
}

fn setup(
    mut commands: Commands,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    fullscreen: Res<FullscreenEnabled>,
) {
    let window = windows.get_primary_mut().unwrap();

    if fullscreen.0 {
        window.set_mode(WindowMode::BorderlessFullscreen);
    } else {
        window.set_mode(WindowMode::Windowed);
    }

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
        crosshair: color_material.add(asset_server.load("crosshair.png").into()),
        font: asset_server.load("font.ttf"),
    })
}

fn get_config() -> Config {
    // Read the file
    let contents = fs::read_to_string("config.json")
    .expect("Something went wrong reading the file");

    let config: Config = serde_json::from_str(contents.as_str()).unwrap();

    config
}

fn main() {
    let config = get_config();

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
        .insert_resource(FullscreenEnabled(config.fullscreen))
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
