use super::super::components::camera;
use super::super::components::player;
use bevy::prelude::*;

pub fn follow_player(
    mut cam: Query<&mut Transform, (With<camera::MainCamera>, Without<player::Player>)>,
    player: Query<&Transform, (With<player::Player>, Without<camera::MainCamera>)>,
) {
    let camera_transform = cam.get_single_mut();
    if let Ok(mut camera_transform) = camera_transform {
        if let Ok(player_transform) = player.get_single() {
            let player_pos = player_transform.translation;
            let smoothness = 0.1;
            let smoothed_position = camera_transform.translation.lerp(player_pos, smoothness);
            camera_transform.translation = smoothed_position;
        }
    }
}
