use std::collections;
use bevy::prelude::*;
use bevy::window::*;
use bevy::utils::Duration;
use collections::HashMap;


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
    pub animation: usize,
    pub animation_len: usize,
    pub animation_duration: Duration
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}


#[derive(Component, Reflect)]
struct Animation {
    texture_atlas: TextureAtlas,
}

#[derive(Component)]
struct Animations (pub HashMap<String,Animation>);

#[derive(Component, Reflect)]
pub struct AnimationTimer(pub Timer);

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

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert(Player {
            speed: 5000.0,
            animation: 1,
            animation_len: 5,
            animation_duration: Duration::from_millis(800)
        })
        .insert(Name::new("Player"));
}

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &mut Player)>,
) {
    let (mut timer, mut sprite, player) = query.single_mut();
    timer.0.set_duration(player.animation_duration);
    timer.0.tick(time.delta());
    if timer.0.finished() {
        sprite.index = ((sprite.index + 1) % player.animation_len) + 6 * player.animation;
    }
}