use bevy::prelude::*;
use bevy::window::*;
use crate::entities::player;

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
        .add_startup_system(spawn_camera)
        .add_startup_system(entities::player::spawn_player)
        .add_system(player::animate_player)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
