use bevy::{prelude::*};

use crate::{GameState, MainScreenRelated, Materials, StartBtn, Title};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                .with_system(main_menu_setup.system()),
            )
            .add_system_set(
                SystemSet::on_resume(GameState::MainMenu)
                .with_system(main_menu_setup.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                .with_system(despawn_title.system())
                .with_system(despawn_start_button.system())
            )
            .add_system_set(
                SystemSet::on_pause(GameState::MainMenu)
                .with_system(despawn_title.system())
                .with_system(despawn_start_button.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                .with_system(switch_to_game.system()),
            );
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
        .insert(MainScreenRelated)
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
            .insert(Title)
            .insert(MainScreenRelated);

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
            .insert(StartBtn)
            .insert(MainScreenRelated);
        });
}

fn despawn_title(
    mut query: Query<Entity, With<MainScreenRelated>>,
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
