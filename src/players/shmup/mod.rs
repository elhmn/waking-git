pub mod components;
pub mod debug;
pub mod plugin;
pub mod systems;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::time::FixedTimestep;
use bevy::{app::PluginGroupBuilder, prelude::*};

const TIMESTEP_60_FPS: f64 = 1. / 60.;

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

pub fn run() {
    App::new()
        .add_plugins(default_plugins())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_60_FPS)),
        )
        .add_plugin(plugin::ShmupPlugin)
        .run();
}
