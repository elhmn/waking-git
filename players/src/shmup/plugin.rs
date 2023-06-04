use super::components::camera;
use super::debug;
use super::placer;
use super::systems::camera as camera_system;
use super::systems::movements;
use super::systems::player as player_systems;
use super::systems::player_bullet;
use super::WorldData;
use bevy::{prelude::*, render::camera::ScalingMode};

pub struct ShmupPlugin;

impl Plugin for ShmupPlugin {
    fn build(&self, app: &mut App) {
        // set GitHub dark mode background color
        let bg_color: Color = Color::hex("0e1117").unwrap_or_default();

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
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(3000.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(camera::MainCamera);

    let mut placer = placer::new();
    placer.spawn_entities(
        windows,
        world_data,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
