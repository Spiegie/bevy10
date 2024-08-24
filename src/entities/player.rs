use core::f32;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::*;
use pid::Pid;


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {}


#[derive(Component)]
pub struct PIDx {
    pub pid: Pid<f32>,
}
#[derive(Component)]
pub struct PIDy {
    pub pid: Pid<f32>,
}
#[derive(Component)]
pub struct PIDr {
    pub pid: Pid<f32>,
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut LinearVelocity, &mut AngularVelocity, &mut Transform), With<Player>>,
    mut pid_query: Query<(&mut PIDx, &mut PIDy, &mut PIDr), With<Player>>
) {
    // queries
    let (mut linear_velocity, mut angular_velocity, transform) = player_query.single_mut();
    let (mut pidx, mut pidy, mut pidr) = pid_query.single_mut();

    // placeholders / variables
    let thrustforce= 1.0;
    let sidethrustforce = 0.5;
    let rotatethrustforce = 0.002;

    // pid controlls
    pidy.pid.output_limit = thrustforce;
    pidx.pid.output_limit = sidethrustforce;
    pidr.pid.output_limit = rotatethrustforce;

    pidy.pid.p(2.0, 100.0);
    pidy.pid.i(1.5, 20.0);

    pidx.pid.p(20.0, 1000.0);
    pidx.pid.i(3.0, 10.0);

    pidr.pid.p(0.008, 900.0);
    pidr.pid.i(0.001, 0.0001);
    pidr.pid.d(2.5,20.0);

    // handling input
    let pressed_left = keyboard_input.pressed(KeyCode::KeyA);
    let pressed_right = keyboard_input.pressed(KeyCode::KeyD);
    let pressed_up = keyboard_input.pressed(KeyCode::KeyW);
    let pressed_down = keyboard_input.pressed(KeyCode::KeyS);
    let rotate_pressed_left = keyboard_input.pressed(KeyCode::KeyE);
    let rotate_pressed_right= keyboard_input.pressed(KeyCode::KeyQ);
    let pressed_rotate_against_movement = keyboard_input.pressed(KeyCode::KeyJ);
    let pressed_rotate_to_level = keyboard_input.pressed(KeyCode::KeyK);
    let pressed_stop_rotation = keyboard_input.pressed(KeyCode::KeyI);
    let pressed_hover = keyboard_input.pressed(KeyCode::KeyL);

    let x_input = -(pressed_left as i8) + pressed_right as i8;
    let y_input = -(pressed_down as i8) + pressed_up as i8;
    let rotation_input = -(rotate_pressed_left as i8) + rotate_pressed_right as i8;
    
    let mut player_input_dir = Vec2::new(x_input as f32, y_input as f32);

    let player_rotation_vec = (transform.rotation * Vec3::X).xy();
    let player_rotation_angle = player_rotation_vec.angle_between(Vec2::X);


    //// big ififelseelse thingy till better Idea strikes
    // hover
    if pressed_hover {
        println!("hovering");
        pidy.pid.setpoint(0.0);
        pidx.pid.setpoint(0.0);

        let outputy = pidy.pid.next_control_output(linear_velocity.y);
        let outputx = pidx.pid.next_control_output(linear_velocity.x);

        let mut controlvec = Vec2::Y.rotate(player_rotation_vec.normalize()) * outputy.output;
        controlvec += player_rotation_vec.normalize() * outputx.output;


        linear_velocity.x += controlvec.x;
        linear_velocity.y += controlvec.y;
    } 
    // player movement
    else if player_input_dir != Vec2::ZERO {

        if y_input > 0 {
            player_input_dir = (Vec2::Y.rotate(player_rotation_vec.normalize())) * thrustforce * y_input as f32;
        } else {
            player_input_dir = (Vec2::Y.rotate(player_rotation_vec.normalize())) * sidethrustforce * y_input as f32;
        }

        player_input_dir += player_rotation_vec.normalize() * sidethrustforce * x_input as f32;

        linear_velocity.x += player_input_dir.x;
        linear_velocity.y += player_input_dir.y;

    }

    //// big ififelseelse thingy till better Idea strikes
    // stop rotation
    if pressed_stop_rotation {
        println!("stoping rotation");
        if angular_velocity.0 >= rotatethrustforce {
            angular_velocity.0 -= rotatethrustforce;
        } else if angular_velocity.0 <= -rotatethrustforce {
            angular_velocity.0 += rotatethrustforce;
        } else {
            angular_velocity.0 = 0.;
        }  
    } 
    // rotate to level
    else if pressed_rotate_to_level {
        println!("rotate to level");
        pidr.pid.setpoint(0.0);

        let outputr = pidr.pid.next_control_output(player_rotation_angle);
        angular_velocity.0 -= outputr.output;
    }
    // rotate retrograde
    else if pressed_rotate_against_movement {

    }
    // use player input
    else {
        angular_velocity.0 += rotation_input as f32 * rotatethrustforce;
    }

    linear_velocity.x -= linear_velocity.x * 0.0001;
    linear_velocity.y -= linear_velocity.y * 0.0001;


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
            Collider::circle(50.0),
            ColliderDensity(0.0),
            Mass(5.0),
            LinearVelocity(Vec2::ZERO),
            AngularVelocity(0.0),
            RayCaster::new(Vec2::ZERO, Dir2::Y)
                .with_max_time_of_impact(50.0),
        ))
        .insert(Player {})
        .insert(Name::new("Player"))
        .insert(PIDx{pid: Pid::new(0.0, 0.0)})
        .insert(PIDy{pid: Pid::new(0.0, 0.0)})
        .insert(PIDr{pid: Pid::new(0.0, 0.0)});
    
}
