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
            camera_transform.translation.x = player_transform.translation.x.round();
            camera_transform.translation.y = player_transform.translation.y.round();
        }
    }
}
