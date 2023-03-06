use super::components::camera;
use super::components::cell;
use super::components::patterns;
use super::components::player;
use super::debug;
use super::systems::movements;
use super::systems::player as player_systems;
use super::systems::player_bullet;
use super::WorldData;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use core::shapes;
use rand::prelude::*;

pub struct ShmupPlugin;

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
            .add_system(player_systems::movement)
            .add_system(player_systems::keyboad_input)
            .add_system(player_systems::mouse_input)
            .add_system(player_bullet::movement)
            .add_system(player_bullet::despawn);
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
        .spawn(Camera2dBundle::default())
        .insert(camera::MainCamera);

    // Spawn the player before anything else
    spawn_player(&mut commands, &mut meshes, &mut materials);

    //Spawn ennemies
    let main_scene = &data.scenes[&data.main_scene];
    for (id, entity) in &main_scene.entities {
        let color = entity.color.replace('#', "");
        match entity.kind.as_str() {
            shapes::CIRCLE => {
                spawn_circle(
                    id.to_string(),
                    color,
                    &mut commands,
                    win,
                    &mut meshes,
                    &mut materials,
                );
            }
            shapes::RECTANGLE => {
                spawn_rectangle(id.to_string(), color, &mut commands, win);
            }
            shapes::TRIANGLE => {
                spawn_triangle(
                    id.to_string(),
                    color,
                    &mut commands,
                    win,
                    &mut meshes,
                    &mut materials,
                );
            }
            shapes::HEXAGON => {
                spawn_hexagon(
                    id.to_string(),
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
                    id.to_string(),
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

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let color = "ffffff";
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(20., 8).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(player::Player {
            speed: 25.,
            hp: 100.,
        })
        .insert(player::Velocity { x: 0.1, y: 0.1 })
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
    id: String,
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
        .insert(patterns::Pattern3 {
            speed: 200.,
            ..default()
        })
        .insert(cell::Cell {
            name: id,
            ..Default::default()
        });
}

fn spawn_hexagon(
    id: String,
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
        .insert(cell::Cell {
            name: id,
            ..Default::default()
        });
}

fn spawn_triangle(
    id: String,
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
        .insert(cell::Cell {
            name: id,
            ..Default::default()
        });
}

fn spawn_rectangle(id: String, color: String, commands: &mut Commands, win: &Window) {
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
        .insert(cell::Cell {
            name: id,
            ..Default::default()
        });
}
