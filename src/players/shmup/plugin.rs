use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct ShmupPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(1., 1., 1.);

impl Plugin for ShmupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system(setup)
            .add_system(move_pattern_1_system)
            .add_system(move_pattern_2_system);
    }
}

#[derive(Component, Debug)]
struct MovementPattern1;

fn move_pattern_1_system(mut query: Query<(&MovementPattern1, &mut Transform)>, time: Res<Time>) {
    for (m, mut t) in query.iter_mut() {
        println!("query: {:#?} | {:#?}", m, t);
        t.translation.x += 100. * time.delta_seconds();
    }
}

#[derive(Component, Debug)]
struct MovementPattern2;

fn move_pattern_2_system(mut query: Query<(&MovementPattern2, &mut Transform)>, time: Res<Time>) {
    for (m, mut t) in query.iter_mut() {
        println!("query: {:#?} | {:#?}", m, t);
        t.translation.x -= 100. * time.delta_seconds();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
            ..default()
        })
        .insert(MovementPattern1);

    // Hexagon
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
            ..default()
        })
        .insert(MovementPattern2);
}
