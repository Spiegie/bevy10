use bevy::prelude::{Component, Query, Reflect, Res, TextureAtlasSprite, Time, Timer};
use std::time::Duration;

#[derive(Component)]
pub struct AnimationController {
    pub animation_information: Vec<AnimationInfo>,
    pub current_animation: usize,
}

impl AnimationController {
    pub fn get_current_duration(&self) -> Duration {
        self.animation_information[self.current_animation].duration
    }
    pub fn get_current_indexes(&self) -> (usize, usize) {
        self.animation_information[self.current_animation].animation_indexes
    }
}

#[derive(Component, Reflect)]
pub struct AnimationInfo {
    //texture_atlas: TextureAtlas,
    pub animation_indexes: (usize, usize),
    pub duration: Duration,
}


#[derive(Component, Reflect)]
pub struct AnimationTimer(pub Timer);

pub fn animate_entity(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &mut AnimationController)>
) {
    let (mut timer, mut sprite, animation_controller) = query.single_mut();
    let (start_index, end_index) = animation_controller.get_current_indexes();
    timer.0.set_duration(animation_controller.get_current_duration());
    timer.0.tick(time.delta());
    if timer.0.finished() {
        if sprite.index >= end_index {
            sprite.index = start_index;
        } else if sprite.index < start_index {
            sprite.index = start_index
        }
        else {
            sprite.index += 1;
        }
    }
}
