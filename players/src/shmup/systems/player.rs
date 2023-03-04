use super::super::components::player;
use bevy::prelude::*;

pub fn movement(
    mut query: Query<(&mut Transform, &player::Velocity, &player::Player), With<player::Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity, player) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds() * player.speed;
        transform.translation.y += velocity.y * time.delta_seconds() * player.speed;
    }
}

pub fn keyboad_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut player::Velocity, &player::Player)>,
) {
    if let Ok((mut velocity, player)) = query.get_single_mut() {
        velocity.x = if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)
        {
            -player.speed
        } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            player.speed
        } else {
            0.
        };

        velocity.y = if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            player.speed
        } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            -player.speed
        } else {
            0.
        };

        //We need to normalize the velocity vector
        //to avoid diagonal movement being faster
        if velocity.x != 0. && velocity.y != 0. {
            velocity.x /= 2_f32.sqrt();
            velocity.y /= 2_f32.sqrt();
        }
    }
}
