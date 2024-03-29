use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Enemy {
    pub id: String,
    pub scene_id: String,
    pub color: String,
    pub hp: i32,
    pub kind: String,
    pub name: String,
    pub speed: f32,
    pub destructible: bool,
    pub shield: String,
}

#[derive(Component, Debug, Default)]
pub struct EnemyCollider {}
