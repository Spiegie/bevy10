use bevy::{ecs::query::QueryData, prelude::*};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct EntityPhysics {
    pub speed: f32,
    pub jump_force: f32,
    pub facing_right: bool,
}
