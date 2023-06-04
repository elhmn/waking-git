pub mod components;
pub mod debug;
pub mod placer;
pub mod plugin;
pub mod systems;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::time::FixedTimestep;
use bevy::{app::PluginGroupBuilder, prelude::*};

use core::converters;

const TIMESTEP_60_FPS: f64 = 1. / 60.;

#[derive(Resource, Default, Debug)]
pub struct WorldData(converters::shmup::Data);

fn default_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set({
        WindowPlugin {
            window: WindowDescriptor {
                title: "waking-git: shoot 'em up".to_string(),
                ..default()
            },
            ..default()
        }
    })
}

pub fn run(data: converters::shmup::Data) {
    App::new()
        .add_plugins(default_plugins())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIMESTEP_60_FPS)))
        .insert_resource(WorldData(data))
        .add_plugin(plugin::ShmupPlugin)
        .run();
}
