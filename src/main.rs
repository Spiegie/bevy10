use bevy::prelude::*;
use bevy::window::*;
use bevy_ecs_tilemap::prelude::*;


mod entities;
mod animation;
mod tilemap;

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
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, spawn_camera)
        //.add_systems(Startup, spawn_tilemap)
        .add_systems(Startup, entities::player::spawn_player)
        .add_systems(Update, animation::animation::animate_entity)
        //.add_systems(Update, swap_texture_or_hide)
        //.add_systems(Update, get_tilemap)
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

fn swap_texture_or_hide(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let texture_a = TilemapTexture::Single(asset_server.load("tiles.png"));
        let texture_b = TilemapTexture::Single(asset_server.load("tiles2.png"));
        for (mut tilemap_tex, _) in &mut query {
            if *tilemap_tex == texture_a {
                *tilemap_tex = texture_b.clone();
            } else {
                *tilemap_tex = texture_a.clone();
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::H) {
        for (_, mut visibility) in &mut query {
            *visibility = match *visibility {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
        }
    }
}
