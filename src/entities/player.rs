
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::*;
use bevy::utils::Duration;

use bevy_rapier2d::{prelude::*};

// use crate::animation::*;
use crate::animation::animation::{AnimationController, AnimationInfo, AnimationTimer};

use super::entities::EntityPhysics;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}


/*pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball.png"),
            ..default()
    mut _commands: Commands,
        },
        Player {},
    ));
}*/

trait MyTextureAtlas {
    fn get_grid_from_texture_atlas(self) -> (usize, usize);
}

impl MyTextureAtlas for TextureAtlas {
    fn get_grid_from_texture_atlas(self) -> (usize, usize) {
        let test =  self.textures[0];
        let sprite_width = test.width();
        let sprite_height = test.height();

        let atlas_size = self.size;
        let atlas_width = atlas_size[0];
        let atlas_height = atlas_size[1];

        ((atlas_width / sprite_width) as usize, (atlas_height / sprite_height) as usize)
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<((&mut EntityPhysics, &mut Velocity), With<Player>)>,
) {

    let (mut player, _ ) = player_query.single_mut();
    let left = keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::D);
    let x_input = -(left as i8) + right as i8;

    if right {
        player.0.facing_right = true;
    }
    if left {
        player.0.facing_right = false;
    }

    let mut player_input_dir = Vec2::new(x_input as f32, 0.0);
    if player_input_dir != Vec2::ZERO {
        player_input_dir /= player_input_dir.length();
    }

    player.1.linvel.x = player_input_dir.x * player.0.speed;

}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let _window = window_query.get_single().unwrap();
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform::from_xyz(0.0,0.0,0.0),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2 { x: 0.0, y: 0.0 },
            angvel: 0.0,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(KinematicCharacterController::default())
        .insert(Ccd::enabled())
        .insert(Collider::ball(12.0))
        .insert(Restitution::coefficient(0.7))
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert(AnimationController {
            /* animation_information: vec![
                AnimationInfo {
                    animation_indexes: (0, 1),
                    duration: Duration::from_millis(800),
                },
                AnimationInfo {
                    animation_indexes: (6, 11),
                    duration: Duration::from_millis(150),
                }
            ], */
            animations: HashMap::from([
                ("walking".to_owned(), AnimationInfo{
                    animation_indexes: (1,6),
                    duration: Duration::from_millis(150),
                    ..Default::default()
                })
            ]),
            current_animation: "walking".to_owned(),
            update_immediate: false,
        })
        .insert(EntityPhysics {
            speed: 100.0,
            jump_force: 20.0,
            facing_right: true,
        })
        .insert(Player {})
        .insert(Name::new("Player"));
}
