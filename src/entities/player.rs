use core::f32;

use avian2d::prelude::*;
use bevy::input::keyboard;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::window::*;
use bevy_egui::egui::emath::easing::linear;

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
    mut player_query: Query<(&mut LinearVelocity, &mut AngularVelocity, &mut Transform), With<Player>>,
) {
    let (mut linear_velocity, mut angular_velocity, transform) = player_query.single_mut();
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let x_input = -(left as i8) + right as i8;
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let y_input = -(down as i8) + up as i8;
    let rotate_left = keyboard_input.pressed(KeyCode::KeyK);
    let rotate_right= keyboard_input.pressed(KeyCode::KeyJ);
    let rotation_input = -(rotate_left as i8) + rotate_right as i8;

    let mut player_input_dir = Vec2::new(x_input as f32, y_input as f32);

    let player_rotation_vec = (transform.rotation * Vec3::X).xy();

    if player_input_dir != Vec2::ZERO {
        player_input_dir = player_input_dir.normalize();
        
        player_input_dir = player_input_dir.rotate(player_rotation_vec);

        println!("{:#?}", y_input);
        player_input_dir += (Vec2::Y.rotate(player_rotation_vec)) * 5. * y_input as f32;

        player_input_dir += player_rotation_vec * 0.5 * x_input as f32;

        linear_velocity.x += player_input_dir.x;
        linear_velocity.y += player_input_dir.y;

    }


    angular_velocity.0 += rotation_input as f32 * 0.03;


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
            ColliderDensity(0.0),
            Mass(5.0),
            LinearVelocity(Vec2::ZERO),
            AngularVelocity(0.0),
            RayCaster::new(Vec2::ZERO, Dir2::Y)
                .with_max_time_of_impact(10.0),
        ))
        .insert(Player {})
        .insert(Name::new("Player"));
    
}
