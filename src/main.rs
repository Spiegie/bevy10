use bevy::prelude::*;
use bevy::window::*;
use bevy_rapier2d::prelude::*;
use entities::player;


mod entities;
mod animation;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, entities::player::spawn_player)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, animation::animation::animate_entity)
        .add_systems(Update, player::move_player)
        .run();
}

pub fn setup_physics(mut commands: Commands) {

    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

}
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let _window = window_query.get_single().unwrap();

    /*commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });*/

    commands.spawn(Camera2dBundle::default());
}
