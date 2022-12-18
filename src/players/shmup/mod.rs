pub mod plugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

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
        .add_plugin(plugin::ShmupPlugin)
        .run();
}
