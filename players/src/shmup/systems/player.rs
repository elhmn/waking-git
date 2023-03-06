use super::super::components::camera;
use super::super::components::player;
use super::super::components::player_bullet::Bullet;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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

pub fn mouse_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mouse: Res<Input<MouseButton>>,
    mut query: Query<&Transform, With<player::Player>>,
    cam: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
) {
    let win = windows.primary();
    let (camera, camera_transform) = cam.single();

    if let Ok(transform) = query.get_single_mut() {
        if mouse.pressed(MouseButton::Left) {
            let pos = win
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
                .unwrap_or_default();

            //Get the direction of the bullet
            let mut direction = Vec2::new(
                pos.x - transform.translation.x,
                pos.y - transform.translation.y,
            );
            direction = direction.normalize();

            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                    material: materials.add(ColorMaterial::from(
                        Color::hex("ffff00").unwrap_or_default(),
                    )),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                })
                .insert(Bullet {
                    direction,
                    ..Default::default()
                });
        }
    }
}
