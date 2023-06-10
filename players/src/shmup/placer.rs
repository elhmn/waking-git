use super::components::enemy;
use super::components::guns;
use super::components::player;
use super::config;
use super::WorldData;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use core::shapes;
use rand::prelude::*;

const BG_MAP_SIZE: u32 = 100;
const BG_MAP_BLOCK_SIZE: u32 = 30;
const BG_GRID_WIDTH: f32 = 1.;

const AREA_BLOCK_SIZE: f32 = 70.;
const AREA_BLOCK_PADDING: f32 = 20.;
const AREA_BLOCK_COL: i32 = 20;
const AREA_BLOCK_ROW: i32 = 17;

type Area = Vec<Vec<usize>>;

pub struct Placer {
    pub area: Area,
    pub number_of_placed_entities: u32,
    pub area_block_size: f32,
    pub area_block_padding: f32,
    pub area_block_col: i32,
    pub area_block_row: i32,
}

pub fn new() -> Placer {
    Placer {
        area: vec![vec![0; AREA_BLOCK_COL as usize]; AREA_BLOCK_ROW as usize],
        number_of_placed_entities: 0,
        area_block_size: AREA_BLOCK_SIZE,
        area_block_padding: AREA_BLOCK_PADDING,
        area_block_col: AREA_BLOCK_COL,
        area_block_row: AREA_BLOCK_ROW,
    }
}

#[allow(clippy::too_many_arguments)]
impl Placer {
    pub fn spawn_entities(
        &mut self,
        windows: Res<Windows>,
        world_data: Res<WorldData>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let _win = windows.primary();
        let data = &world_data.0;

        self.spawn_game_area(commands);

        // Spawn the player before anything else
        self.spawn_player(commands, meshes, materials);
        self.number_of_placed_entities += 1;
        self.area[(self.area_block_row / 2) as usize][(self.area_block_col / 2) as usize] =
            self.number_of_placed_entities as usize;

        //Spawn ennemies
        let main_scene = &data.scenes[&data.main_scene];
        for entity in main_scene.entities.values() {
            //get possible position
            let mut area_pos = self.random_position();
            let mut position = self.area_pos_to_world_pos(area_pos);
            let mut size = random_size();
            while self.is_occupied(area_pos) || self.mark_occupied_area(area_pos, position, size) {
                area_pos = self.random_position();
                position = self.area_pos_to_world_pos(area_pos);
                size = random_size();
            }

            let color = entity.color.replace('#', "");
            match entity.kind.as_str() {
                shapes::CIRCLE => {
                    self.spawn_circle(
                        entity.name.to_string(),
                        color,
                        size,
                        position,
                        commands,
                        meshes,
                        materials,
                    );
                }
                shapes::RECTANGLE => {
                    self.spawn_rectangle(entity.name.to_string(), color, size, position, commands);
                }
                shapes::TRIANGLE => {
                    self.spawn_triangle(
                        entity.name.to_string(),
                        color,
                        size,
                        position,
                        commands,
                        meshes,
                        materials,
                    );
                }
                shapes::HEXAGON => {
                    self.spawn_hexagon(
                        entity.name.to_string(),
                        color,
                        size,
                        position,
                        commands,
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
                        size,
                        position,
                        commands,
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
                    (self.area_block_size + self.area_block_padding * 2.)
                        * self.area_block_col as f32,
                    (self.area_block_size + self.area_block_padding * 2.)
                        * self.area_block_row as f32,
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
        let player_size = 50.;
        let col_sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
            sprite: Sprite {
                color: Color::hex(config::get_col_color()).unwrap_or_default(),
                custom_size: Some(Vec2::new(player_size, player_size)),
                ..default()
            },
            ..default()
        };

        let color = "26a641";
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
                material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(col_sprite)
                    .insert(player::PlayerCollider::default());
            })
            .insert(player::Player {
                speed: 25.,
                hp: 100.,
            })
            .insert(player::Velocity { x: 0.1, y: 0.1 })
            .insert(player::Gun::default())
            .insert(Name::new("Player"))
            .insert(enemy::Enemy {
                name: "player".to_string(),
                ..Default::default()
            });
    }

    fn spawn_circle(
        &self,
        name: String,
        color: String,
        size: f32,
        position: Vec3,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let col_sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
            sprite: Sprite {
                color: Color::hex(config::get_col_color()).unwrap_or_default(),
                custom_size: Some(Vec2::new(size * 2., size * 2.)),
                ..default()
            },
            ..default()
        };

        let entity = &mut commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
            transform: Transform::from_translation(position),
            ..default()
        });

        entity
            //                         .insert(patterns::MoveTowards::default())
            .insert(Name::new(name.to_owned()))
            .insert(enemy::Enemy {
                name,
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn(col_sprite)
                    .insert(enemy::EnemyCollider::default());
            });

        if size >= 60. {
            entity.insert(guns::MultiDirectionCircleGun::default());
        } else {
            entity.insert(guns::FastGun::default());
        }
    }

    fn spawn_hexagon(
        &self,
        name: String,
        color: String,
        size: f32,
        position: Vec3,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let col_sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
            sprite: Sprite {
                color: Color::hex(config::get_col_color()).unwrap_or_default(),
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            ..default()
        };

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(size, 6).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
                transform: Transform::from_translation(position),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(col_sprite)
                    .insert(enemy::EnemyCollider::default());
            })
            //             .insert(patterns::Pattern3 {
            //                 speed: 150.,
            //                 ..default()
            //})
            .insert(Name::new(name.to_owned()))
            .insert(enemy::Enemy {
                name,
                ..Default::default()
            });
    }

    fn spawn_triangle(
        &self,
        name: String,
        color: String,
        size: f32,
        position: Vec3,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let col_sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
            sprite: Sprite {
                color: Color::hex(config::get_col_color()).unwrap_or_default(),
                custom_size: Some(Vec2::new(size * 1.5, size * 1.5)),
                ..default()
            },
            ..default()
        };

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(size, 3).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::hex(color).unwrap_or_default())),
                transform: Transform::from_translation(position),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(col_sprite)
                    .insert(enemy::EnemyCollider::default());
            })
            .insert(guns::SimpleGun::default())
            //             .insert(patterns::Pattern3 {
            //                 speed: 150.,
            //                 ..default()
            //             })
            .insert(Name::new(name.to_owned()))
            .insert(enemy::Enemy {
                name,
                ..Default::default()
            });
    }

    fn spawn_rectangle(
        &self,
        name: String,
        color: String,
        size: f32,
        position: Vec3,
        commands: &mut Commands,
    ) {
        let padding = 20.;
        let col_sprite = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
            sprite: Sprite {
                color: Color::hex(config::get_col_color()).unwrap_or_default(),
                custom_size: Some(Vec2::new(size + padding + 2., size + padding + 2.)),
                ..default()
            },
            ..default()
        };

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex(color).unwrap_or_default(),
                    custom_size: Some(Vec2::new(size + padding, size + padding)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(col_sprite)
                    .insert(enemy::EnemyCollider::default());
            })
            .insert(guns::MultiDirectionRectangleGun::default()) // Debug
            //             .insert(patterns::Pattern3 {
            //                 speed: 100.,
            //                 ..default()
            //             })
            .insert(Name::new(name.to_owned()))
            .insert(enemy::Enemy {
                name,
                ..Default::default()
            });
    }

    fn random_position(&self) -> Vec2 {
        let mut r = thread_rng();
        let x = r.gen_range(0..(self.area_block_col));
        let y = r.gen_range(0..(self.area_block_row));

        //if the there is something on that position, try again
        Vec2::new(x as f32, y as f32)
    }

    fn is_occupied(&self, position: Vec2) -> bool {
        self.area[position.y as usize][position.x as usize] != 0
    }

    //mark areas affected by an entity as occupied
    //it uses the size of the entity to mark the areas it
    //touches as occupied
    fn mark_occupied_area(&mut self, area_pos: Vec2, position: Vec3, size: f32) -> bool {
        let size = size / 2.;
        let _a = &mut self.area;
        let i = area_pos.y as usize;
        let j = area_pos.x as usize;

        let col = self.area_block_col as f32;
        let row = self.area_block_row as f32;
        let position = position
            + Vec3::new(
                (self.area_block_size + self.area_block_padding) * col / 2.,
                (self.area_block_size + self.area_block_padding) * row / 2.,
                0.,
            );
        let left = position + Vec3::new(-size, 0., 0.);
        let right = position + Vec3::new(size, 0., 0.);
        let top = position + Vec3::new(0., size, 0.);
        let bottom = position + Vec3::new(0., -size, 0.);
        let top_left = position + Vec3::new(-size, size, 0.);
        let top_right = position + Vec3::new(size, size, 0.);
        let bottom_left = position + Vec3::new(-size, -size, 0.);
        let bottom_right = position + Vec3::new(size, -size, 0.);

        let multiple = self.area_block_size + self.area_block_padding;
        let a_size = Vec3::new(multiple, multiple, 1.);
        let a_left_pos = left / a_size;
        let a_right_pos = right / a_size;
        let a_top_pos = top / a_size;
        let a_bottom_pos = bottom / a_size;
        let a_top_left_pos = top_left / a_size;
        let a_top_right_pos = top_right / a_size;
        let a_bottom_left_pos = bottom_left / a_size;
        let a_bottom_right_pos = bottom_right / a_size;

        //Check that the area is not occupied
        if self.is_occupied(a_left_pos.truncate())
            || self.is_occupied(a_right_pos.truncate())
            || self.is_occupied(a_top_pos.truncate())
            || self.is_occupied(a_bottom_pos.truncate())
            || self.is_occupied(a_top_left_pos.truncate())
            || self.is_occupied(a_top_right_pos.truncate())
            || self.is_occupied(a_bottom_left_pos.truncate())
            || self.is_occupied(a_bottom_right_pos.truncate())
        {
            return true;
        }

        //Occupy areas
        self.number_of_placed_entities += 1;
        self.area[i][j] = self.number_of_placed_entities as usize;
        self.occupy(a_left_pos.x, a_left_pos.y);
        self.occupy(a_right_pos.x, a_right_pos.y);
        self.occupy(a_top_pos.x, a_top_pos.y);
        self.occupy(a_bottom_pos.x, a_bottom_pos.y);
        self.occupy(a_top_left_pos.x, a_top_left_pos.y);
        self.occupy(a_top_right_pos.x, a_top_right_pos.y);
        self.occupy(a_bottom_left_pos.x, a_bottom_left_pos.y);
        self.occupy(a_bottom_right_pos.x, a_bottom_right_pos.y);

        false
    }

    fn area_pos_to_world_pos(&self, position: Vec2) -> Vec3 {
        Vec3::new(
            position.x * (self.area_block_size + self.area_block_padding)
                - (self.area_block_size + self.area_block_padding) * self.area_block_col as f32
                    / 2.,
            position.y * (self.area_block_size + self.area_block_padding)
                - (self.area_block_size + self.area_block_padding) * self.area_block_row as f32
                    / 2.,
            0.,
        )
    }

    fn occupy(&mut self, x: f32, y: f32) {
        let x = if x as usize >= self.area_block_col as usize {
            (self.area_block_col - 1) as usize
        } else {
            x as usize
        };

        let y = if y as usize >= self.area_block_row as usize {
            (self.area_block_row - 1) as usize
        } else {
            y as usize
        };

        self.area[y][x] = self.number_of_placed_entities as usize;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.area_block_row as usize {
            for j in 0..self.area_block_col as usize {
                print!("{:3}", self.area[i][j]);
            }
            println!();
        }
    }
}

fn random_size() -> f32 {
    match rand::thread_rng().gen_range(0..10) {
        v if (0..1).contains(&v) => 40.,
        v if (1..2).contains(&v) => 50.,
        v if (2..3).contains(&v) => 60.,
        v if (3..4).contains(&v) => 70.,
        v if (4..5).contains(&v) => 80.,
        _ => 30.,
    }
}
