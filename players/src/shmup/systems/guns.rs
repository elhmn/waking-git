use super::super::components::enemy_bullets::{
    self, BulletKind, COLOR_DESTRUCTIBLE, COLOR_UNDESTRUCTIBLE,
};
use super::super::components::guns::{self, Direction};
use super::super::components::player;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::time::Duration;

pub fn simple_gun(
    mut commands: Commands,
    mut gun_query: Query<(&mut Transform, &mut guns::SimpleGun)>,
    player: Query<&Transform, (With<player::Player>, Without<guns::SimpleGun>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let player = player.get_single().unwrap();

    for (gun_transform, mut gun) in gun_query.iter_mut() {
        let direction = player.translation - gun_transform.translation;
        let direction = direction.normalize().truncate();
        if gun.cooldown_timer.finished() {
            spawn_bullet(
                &mut commands,
                *gun_transform,
                direction,
                700.,
                BulletKind::Destructible,
                &mut meshes,
                &mut materials,
            );
            gun.cooldown_timer.reset();
        } else {
            gun.cooldown_timer
                .tick(Duration::from_secs_f32(time.delta_seconds()));
        }
    }
}

pub fn fast_gun(
    mut commands: Commands,
    mut gun_query: Query<(&mut Transform, &mut guns::FastGun)>,
    player: Query<&Transform, (With<player::Player>, Without<guns::FastGun>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let player = player.get_single().unwrap();

    for (gun_transform, mut gun) in gun_query.iter_mut() {
        let direction = player.translation - gun_transform.translation;
        let direction = direction.normalize().truncate();
        if gun.cooldown_timer.finished() {
            if gun.count_fired_bullets <= 2 {
                //shoot orange bullets
                spawn_bullet(
                    &mut commands,
                    *gun_transform,
                    direction,
                    700.,
                    BulletKind::Destructible,
                    &mut meshes,
                    &mut materials,
                );
            } else {
                //shoot undestructible bullets
                spawn_bullet(
                    &mut commands,
                    *gun_transform,
                    direction,
                    700.,
                    BulletKind::Undestructible,
                    &mut meshes,
                    &mut materials,
                );
            }
            gun.count_fired_bullets = if gun.count_fired_bullets > 4 {
                0
            } else {
                gun.count_fired_bullets + 1
            };

            gun.cooldown_timer.reset();
        } else {
            gun.cooldown_timer
                .tick(Duration::from_secs_f32(time.delta_seconds()));
        }
    }
}

pub fn multidirection_circle_gun(
    mut commands: Commands,
    mut gun_query: Query<(&mut Transform, &mut guns::MultiDirectionCircleGun)>,
    player: Query<&Transform, (With<player::Player>, Without<guns::MultiDirectionCircleGun>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let player = player.get_single().unwrap();

    for (gun_transform, mut gun) in gun_query.iter_mut() {
        let direction = player.translation - gun_transform.translation;
        let north_dir = direction.normalize().truncate();
        if gun.cooldown_timer.finished() {
            for dir in gun.directions.iter() {
                match dir {
                    Direction::North => {
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            north_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                    Direction::NorthEast => {
                        let rad_angle = -std::f32::consts::PI / 4.;
                        let angle = Vec2::new(rad_angle.cos(), rad_angle.sin());
                        let north_east_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            north_east_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }

                    Direction::NorthWest => {
                        let rad_angle = std::f32::consts::PI / 4.;
                        let angle = Vec2::new(rad_angle.cos(), rad_angle.sin());
                        let north_west_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            north_west_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }

                    Direction::South => {
                        let angle = Vec2::new(-1., 0.);
                        let south_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            south_dir.normalize(),
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }

                    Direction::SouthEast => {
                        let rad_angle = -3. * std::f32::consts::PI / 4.;
                        let angle = Vec2::new(rad_angle.cos(), rad_angle.sin());
                        let south_east_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            south_east_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }

                    Direction::SouthWest => {
                        let rad_angle = 3. * std::f32::consts::PI / 4.;
                        let angle = Vec2::new(rad_angle.cos(), rad_angle.sin());
                        let south_west_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            south_west_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                    Direction::East => {
                        let angle = Vec2::new(0., -1.);
                        let east_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            east_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                    Direction::West => {
                        let angle = Vec2::new(0., 1.);
                        let west_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            west_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                };
            }
            gun.cooldown_timer.reset();
        } else {
            gun.cooldown_timer
                .tick(Duration::from_secs_f32(time.delta_seconds()));
        }
    }
}

pub fn multidirection_rectangle_gun(
    mut commands: Commands,
    mut gun_query: Query<(&mut Transform, &mut guns::MultiDirectionRectangleGun)>,
    player: Query<
        &Transform,
        (
            With<player::Player>,
            Without<guns::MultiDirectionRectangleGun>,
        ),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let player = player.get_single().unwrap();

    for (gun_transform, mut gun) in gun_query.iter_mut() {
        let direction = player.translation - gun_transform.translation;
        let north_dir = direction.normalize().truncate();
        if gun.cooldown_timer.finished() {
            for dir in gun.directions.iter() {
                match dir {
                    Direction::North => {
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            north_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                    Direction::South => {
                        let angle = Vec2::new(-1., 0.);
                        let south_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            south_dir.normalize(),
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                    Direction::East => {
                        let angle = Vec2::new(0., -1.);
                        let east_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            east_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                    Direction::West => {
                        let angle = Vec2::new(0., 1.);
                        let west_dir = angle.rotate(north_dir);
                        spawn_bullet(
                            &mut commands,
                            *gun_transform,
                            west_dir,
                            700.,
                            BulletKind::Undestructible,
                            &mut meshes,
                            &mut materials,
                        );
                    }
                    _ => (),
                };
            }
            gun.cooldown_timer.reset();
        } else {
            gun.cooldown_timer
                .tick(Duration::from_secs_f32(time.delta_seconds()));
        }
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    position: Transform,
    direction: Vec2,
    speed: f32,
    kind: BulletKind,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let color = if matches!(kind, BulletKind::Destructible) {
        COLOR_DESTRUCTIBLE
    } else {
        COLOR_UNDESTRUCTIBLE
    };

    let size = 20.;
    let col_color = "26a64166";
    let col_sprite = SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
        sprite: Sprite {
            color: Color::hex(col_color).unwrap_or_default(),
            custom_size: Some(Vec2::new(size * 2., size * 2.)),
            ..default()
        },
        ..default()
    };

    let mut entity = commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(size).into()).into(),
        material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
        transform: position,
        ..default()
    });

    entity
        .with_children(|parent| {
            let mut child = parent.spawn(col_sprite);

            if matches!(kind, BulletKind::Destructible) {
                child.insert(enemy_bullets::DestructibleBullet::default());
            } else {
                child.insert(enemy_bullets::UndestructibleBullet::default());
            }

            child.insert(enemy_bullets::BulletCollider::default());
        })
        .insert(enemy_bullets::Bullet {
            direction,
            speed,
            ..Default::default()
        });
}
