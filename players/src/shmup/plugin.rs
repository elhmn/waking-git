use super::components::camera;
use super::components::cell;
use super::components::patterns;
use super::components::player;
use super::debug;
use super::systems::camera as camera_system;
use super::systems::movements;
use super::systems::player as player_systems;
use super::systems::player_bullet;
use super::WorldData;
use bevy::{prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle};
use core::shapes;
use rand::prelude::*;

pub struct ShmupPlugin;

const MAP_SIZE: u32 = 100;
const MAP_BLOCK_SIZE: u32 = 30;
const GRID_WIDTH: f32 = 0.3;

impl Plugin for ShmupPlugin {
    fn build(&self, app: &mut App) {
        // set GitHub dark mode background color
        let bg_color: Color = Color::hex("2d333b").unwrap_or_default();

        app.insert_resource(ClearColor(bg_color))
            .add_plugin(debug::DebugPlugin)
            .add_startup_system(setup)
            .add_startup_system(movements::pattern_3_init)
            .add_system(movements::movement_pattern_1)
            .add_system(movements::movement_pattern_2)
            .add_system(movements::movement_pattern_3)
            .add_system(movements::move_towards)
            .add_system(player_systems::movement)
            .add_system(player_systems::keyboad_input)
            .add_system(player_systems::mouse_input)
            .add_system(player_bullet::movement)
            .add_system(player_bullet::despawn)
            .add_system(camera_system::follow_player);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    world_data: Res<WorldData>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let win = windows.primary();
    let data = &world_data.0;

    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(1500.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(camera::MainCamera);

    spawn_background(&mut commands);

    // Spawn the player before anything else
    spawn_player(&mut commands, &mut meshes, &mut materials);

    //Spawn ennemies
    let main_scene = &data.scenes[&data.main_scene];
    for entity in main_scene.entities.values() {
        let color = entity.color.replace('#', "");
        match entity.kind.as_str() {
            shapes::CIRCLE => {
                spawn_circle(
                    entity.name.to_string(),
                    color,
                    &mut commands,
                    win,
                    &mut meshes,
                    &mut materials,
                );
            }
            shapes::RECTANGLE => {
                spawn_rectangle(entity.name.to_string(), color, &mut commands, win);
            }
            shapes::TRIANGLE => {
                spawn_triangle(
                    entity.name.to_string(),
                    color,
                    &mut commands,
                    win,
                    &mut meshes,
                    &mut materials,
                );
            }
            shapes::HEXAGON => {
                spawn_hexagon(
                    entity.name.to_string(),
                    color,
                    &mut commands,
                    win,
                    &mut meshes,
                    &mut materials,
                );
            }
            _ => {
                //we will spawn an hexagon until we have defined
                //a different shape for the rest of entity's kind
                spawn_hexagon(
                    entity.name.to_string(),
                    color,
                    &mut commands,
                    win,
                    &mut meshes,
                    &mut materials,
                );
            }
        };
    }
}

fn spawn_background(commands: &mut Commands) {
    //took from that tutorial https://johanhelsing.studio/posts/extreme-bevy-2
    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.,
                (MAP_SIZE as f32 * MAP_BLOCK_SIZE as f32 / 2.) - (i as f32 * MAP_BLOCK_SIZE as f32),
                -0.1,
            )),
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(
                    MAP_BLOCK_SIZE as f32 * MAP_SIZE as f32,
                    GRID_WIDTH,
                )),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                (MAP_SIZE as f32 * MAP_BLOCK_SIZE as f32 / 2.) - (i as f32 * MAP_BLOCK_SIZE as f32),
                0.,
                -0.1,
            )),
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(
                    GRID_WIDTH,
                    MAP_BLOCK_SIZE as f32 * MAP_SIZE as f32,
                )),
                ..default()
            },
            ..default()
        });
    }
}

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let color = "26a641";
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(player::Player {
            speed: 25.,
            hp: 100.,
        })
        .insert(player::Velocity { x: 0.1, y: 0.1 })
        .insert(player::Gun::default())
        .insert(Name::new("Player"))
        .insert(cell::Cell {
            name: "player".to_string(),
            ..Default::default()
        });
}

fn get_random_position(w: f32, h: f32) -> Vec3 {
    let mut r = thread_rng();
    let x = r.gen_range((-w / 2.)..w / 2.);
    let y = r.gen_range((-h / 2.)..h / 2.);

    Vec3::new(x, y, 0.)
}

fn spawn_circle(
    name: String,
    color: String,
    commands: &mut Commands,
    win: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
            transform: Transform::from_translation(get_random_position(win.width(), win.height())),
            ..default()
        })
        .insert(patterns::MoveTowards::default())
        .insert(Name::new(name.to_owned()))
        .insert(cell::Cell {
            name,
            ..Default::default()
        });
}

fn spawn_hexagon(
    name: String,
    color: String,
    commands: &mut Commands,
    win: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(20., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
            transform: Transform::from_translation(get_random_position(win.width(), win.height())),
            ..default()
        })
        .insert(patterns::Pattern3 {
            speed: 150.,
            ..default()
        })
        .insert(Name::new(name.to_owned()))
        .insert(cell::Cell {
            name,
            ..Default::default()
        });
}

fn spawn_triangle(
    name: String,
    color: String,
    commands: &mut Commands,
    win: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
            transform: Transform::from_translation(get_random_position(win.width(), win.height())),
            ..default()
        })
        .insert(patterns::Pattern3 {
            speed: 150.,
            ..default()
        })
        .insert(Name::new(name.to_owned()))
        .insert(cell::Cell {
            name,
            ..Default::default()
        });
}

fn spawn_rectangle(name: String, color: String, commands: &mut Commands, win: &Window) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::hex(color).unwrap_or_default(),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_translation(get_random_position(win.width(), win.height())),
            ..default()
        })
        .insert(patterns::Pattern3 {
            speed: 100.,
            ..default()
        })
        .insert(Name::new(name.to_owned()))
        .insert(cell::Cell {
            name,
            ..Default::default()
        });
}
