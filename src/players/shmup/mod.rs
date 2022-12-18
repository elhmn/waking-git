use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(Component, Debug)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Cirah".to_string())));
    commands.spawn((Person, Name("Kira".to_string())));
}

fn move_system(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("I am moving: {:#?}", name.0);
    }
}

pub struct ShmupPlugin;

impl Plugin for ShmupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_startup_system(add_people)
            .add_system(move_system);
    }
}

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
        .add_plugin(ShmupPlugin)
        .run();
}
