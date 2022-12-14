use super::components::cell;
use super::components::patterns;
use super::debug;
use super::systems::movements;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for i in 1..1000 {
        // Circle
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
                ..default()
            })
            .insert(patterns::Pattern3 {
                speed: 200.,
                ..default()
            })
            .insert(cell::Cell {
                name: format!("{}", i),
                ..Default::default()
            });

        // Hexagon
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 6).into()).into(),
                material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                transform: Transform::from_translation(Vec3::new(100., 200., 0.)),
                ..default()
            })
            .insert(patterns::Pattern3 {
                speed: 150.,
                ..default()
            });

        // Rectangle
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                ..default()
            })
            .insert(patterns::Pattern3 {
                speed: 100.,
                ..default()
            })
            .insert(cell::Cell {
                name: "hello".to_string(),
                ..Default::default()
            });
    }
}
