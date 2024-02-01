use bevy::{prelude::{Component, Query, Reflect, Res, TextureAtlasSprite, Time, Timer}, utils::HashMap};
use std::time::Duration;

use crate::entities::{entities::EntityPhysics};

#[derive(Component, Reflect, Copy, Clone)]
pub struct AnimationInfo {
    //texture_atlas: TextureAtlas,
    pub animation_indexes: (usize, usize),
    pub duration: Duration,
    pub onetime: bool,
    pub looping: bool,
}

impl Default for AnimationInfo {
    fn default() -> Self {
        AnimationInfo {
            animation_indexes: (0,0),
            duration: Duration::from_millis(100),
            onetime: false,
            looping: true,
        }
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub animations: HashMap<String, AnimationInfo>,
    pub current_animation: String,
    pub update_immediate: bool,
}



impl AnimationController {
    pub fn get_animation_info(&self) -> (Duration, (usize, usize)) {
        let animation_info = self.animations.get(&self.current_animation).unwrap();
        (animation_info.duration, animation_info.animation_indexes )
    }
    pub fn _reset(&mut self) {
        self.update_immediate = true;
    }
}

#[derive(Component, Reflect)]
pub struct AnimationTimer(pub Timer);

pub fn animate_entity(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &mut AnimationController, &EntityPhysics)>
) {

    for (mut timer, mut sprite, mut animation_controller, entity_physics) in &mut query {
        let (duration, (start_index, end_index)) = animation_controller.get_animation_info();
        timer.0.set_duration(duration);
        timer.0.tick(time.delta());
        if animation_controller.update_immediate {
            timer.0.set_elapsed(duration);
            animation_controller.update_immediate = false
        }
        if timer.0.just_finished() {
            sprite.index = if sprite.index >= end_index || sprite.index < start_index {
                start_index
            } else {
                sprite.index + 1
            }
        }

        sprite.flip_x = !entity_physics.facing_right;
    }
}
