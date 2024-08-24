use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::*;
use entities::player;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod entities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy10".to_owned(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 25.0))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, entities::player::spawn_player)
        .add_systems(Update, player::move_player)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let _window = window_query.get_single().unwrap();

    /*commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });*/

    commands.spawn(Camera2dBundle::default());
}
