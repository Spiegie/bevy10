use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::*;

use super::entities::EntityPhysics;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    let mut player = player_query.single_mut();
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let x_input = -(left as i8) + right as i8;
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let y_input = -(down as i8) + up as i8;

    let mut player_input_dir = Vec2::new(x_input as f32, 0.0);
    if player_input_dir != Vec2::ZERO {
        player_input_dir /= player_input_dir.length();
    }
}

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let _window = window_query.get_single().unwrap();
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(0.0, 0.0, 0.0),
                global: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::circle(10.0),
        ))
        .insert(Player {})
        .insert(Name::new("Player"));
}
