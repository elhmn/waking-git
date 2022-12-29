use super::components::cell;
use super::components::patterns;
use super::debug;
use super::systems::movements;
use super::WorldData;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

pub struct ShmupPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);

impl Plugin for ShmupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_plugin(debug::DebugPlugin)
            .add_startup_system(setup)
            .add_startup_system(movements::pattern_3_init)
            .add_system(movements::movement_pattern_1)
            .add_system(movements::movement_pattern_2)
            .add_system(movements::movement_pattern_3);
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
    commands.spawn(Camera2dBundle::default());

    //The scenes are traversed in a random order,
    //which means that the first scene displayed will be different
    //every time we run the program.
    //This also means that the first scene won't correspond to
    //the git repository root tree.
    //
    //To solve that we will later have to add a `data.root_scene` field
    //to the `converters::shmup::Data` struct, so that we know what is the
    //initial scene to display.
    for scene in data.scenes.values() {
        for entity_id in scene.entities.keys() {
            spawn_circle(
                entity_id.to_string(),
                &mut commands,
                win,
                &mut meshes,
                &mut materials,
            );

            spawn_hexagon(
                entity_id.to_string(),
                &mut commands,
                win,
                &mut meshes,
                &mut materials,
            );

            spawn_rectangle(entity_id.to_string(), &mut commands, win);
        }
    }
}

fn get_random_position(w: f32, h: f32) -> Vec3 {
    let mut r = thread_rng();
    let x = r.gen_range((-w / 2.)..w / 2.);
    let y = r.gen_range((-h / 2.)..h / 2.);

    Vec3::new(x, y, 0.)
}

fn spawn_circle(
    id: String,
    commands: &mut Commands,
    win: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
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
    commands: &mut Commands,
    win: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(10., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
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

fn spawn_rectangle(id: String, commands: &mut Commands, win: &Window) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(20.0, 20.0)),
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
