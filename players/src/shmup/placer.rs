use super::components::cell;
use super::components::patterns;
use super::components::player;
use super::WorldData;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use core::shapes;
use rand::prelude::*;

const BG_MAP_SIZE: u32 = 100;
const BG_MAP_BLOCK_SIZE: u32 = 30;
const BG_GRID_WIDTH: f32 = 0.3;

const AREA_BLOCK_SIZE: f32 = 50.;
const AREA_BLOCK_PADDING: f32 = 10.;
const AREA_BLOCK_COL: f32 = 17.;
const AREA_BLOCK_ROW: f32 = 14.;

type Area = Vec<Vec<bool>>;

pub struct Placer {
    pub area: Area,
}

pub fn new() -> Placer {
    Placer {
        area: vec![vec![false; AREA_BLOCK_COL as usize]; AREA_BLOCK_ROW as usize],
    }
}

impl Placer {
    pub fn spawn_entities(
        &self,
        windows: Res<Windows>,
        world_data: Res<WorldData>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let win = windows.primary();
        let data = &world_data.0;

        self.spawn_game_area(commands);

        // Spawn the player before anything else
        self.spawn_player(commands, meshes, materials);

        //Spawn ennemies
        let main_scene = &data.scenes[&data.main_scene];
        for entity in main_scene.entities.values() {
            let color = entity.color.replace('#', "");
            match entity.kind.as_str() {
                shapes::CIRCLE => {
                    self.spawn_circle(
                        entity.name.to_string(),
                        color,
                        random_size(),
                        commands,
                        win,
                        meshes,
                        materials,
                    );
                }
                shapes::RECTANGLE => {
                    self.spawn_rectangle(
                        entity.name.to_string(),
                        color,
                        random_rectangle_size(),
                        commands,
                        win,
                    );
                }
                shapes::TRIANGLE => {
                    self.spawn_triangle(
                        entity.name.to_string(),
                        color,
                        random_size(),
                        commands,
                        win,
                        meshes,
                        materials,
                    );
                }
                shapes::HEXAGON => {
                    self.spawn_hexagon(
                        entity.name.to_string(),
                        color,
                        random_size(),
                        commands,
                        win,
                        meshes,
                        materials,
                    );
                }
                _ => {
                    //we will spawn an hexagon until we have defined
                    //a different shape for the rest of entity's kind
                    self.spawn_hexagon(
                        entity.name.to_string(),
                        color,
                        random_size(),
                        commands,
                        win,
                        meshes,
                        materials,
                    );
                }
            };
        }
    }

    fn spawn_game_area(&self, commands: &mut Commands) {
        let color: Color = Color::hex("2d333b").unwrap_or_default();
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., -0.2)),
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(
                    AREA_BLOCK_SIZE * (AREA_BLOCK_COL + AREA_BLOCK_PADDING * 2.),
                    AREA_BLOCK_SIZE * (AREA_BLOCK_ROW + AREA_BLOCK_PADDING * 2.),
                )),
                ..default()
            },
            ..default()
        });
    }

    #[allow(dead_code)]
    fn spawn_background(&self, commands: &mut Commands) {
        //took from that tutorial https://johanhelsing.studio/posts/extreme-bevy-2
        // Horizontal lines
        for i in 0..=BG_MAP_SIZE {
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    0.,
                    (BG_MAP_SIZE as f32 * BG_MAP_BLOCK_SIZE as f32 / 2.)
                        - (i as f32 * BG_MAP_BLOCK_SIZE as f32),
                    -0.1,
                )),
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(
                        BG_MAP_BLOCK_SIZE as f32 * BG_MAP_SIZE as f32,
                        BG_GRID_WIDTH,
                    )),
                    ..default()
                },
                ..default()
            });
        }

        // Vertical lines
        for i in 0..=BG_MAP_SIZE {
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    (BG_MAP_SIZE as f32 * BG_MAP_BLOCK_SIZE as f32 / 2.)
                        - (i as f32 * BG_MAP_BLOCK_SIZE as f32),
                    0.,
                    -0.1,
                )),
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(
                        BG_GRID_WIDTH,
                        BG_MAP_BLOCK_SIZE as f32 * BG_MAP_SIZE as f32,
                    )),
                    ..default()
                },
                ..default()
            });
        }
    }

    fn spawn_player(
        &self,
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

    fn spawn_circle(
        &self,
        name: String,
        color: String,
        size: f32,
        commands: &mut Commands,
        win: &Window,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(size).into()).into(),
                material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
                transform: Transform::from_translation(get_random_position(
                    win.width(),
                    win.height(),
                )),
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
        &self,
        name: String,
        color: String,
        size: f32,
        commands: &mut Commands,
        win: &Window,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(size, 6).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
                transform: Transform::from_translation(get_random_position(
                    win.width(),
                    win.height(),
                )),
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
        &self,
        name: String,
        color: String,
        size: f32,
        commands: &mut Commands,
        win: &Window,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(size, 3).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
                transform: Transform::from_translation(get_random_position(
                    win.width(),
                    win.height(),
                )),
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

    fn spawn_rectangle(
        &self,
        name: String,
        color: String,
        size: f32,
        commands: &mut Commands,
        win: &Window,
    ) {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex(color).unwrap_or_default(),
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                transform: Transform::from_translation(get_random_position(
                    win.width(),
                    win.height(),
                )),
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
}

fn random_size() -> f32 {
    match rand::thread_rng().gen_range(0..10) {
        v if (0..1).contains(&v) => 30.,
        v if (1..2).contains(&v) => 40.,
        v if (2..3).contains(&v) => 50.,
        v if (3..4).contains(&v) => 60.,
        v if (4..5).contains(&v) => 70.,
        _ => 20.,
    }
}

fn random_rectangle_size() -> f32 {
    match rand::thread_rng().gen_range(0..10) {
        v if (0..1).contains(&v) => 50.,
        v if (1..2).contains(&v) => 60.,
        v if (2..3).contains(&v) => 70.,
        v if (3..4).contains(&v) => 80.,
        v if (4..5).contains(&v) => 90.,
        _ => 40.,
    }
}

fn get_random_position(w: f32, h: f32) -> Vec3 {
    let mut r = thread_rng();
    let x = r.gen_range((-w / 2.)..w / 2.);
    let y = r.gen_range((-h / 2.)..h / 2.);

    Vec3::new(x, y, 0.)
}
