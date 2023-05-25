/*
    Bevy Getting Started Example
    https://bevyengine.org/learn/book/getting-started

    The Bevy Engine is a data-driven game engine built in Rust.
    It uses Bevy ECS, a custom Entity Component System.

    This app will create a window and print welcome messages every
    two seconds.

    Some Notes:

    - Bevy programs are referred to as `Apps`
    - Engine features are implemented as `plugins`... Some examples:
      - `UiPlugin`
      - `RenderPlugin`
      - `WindowPlugin`
    - Resources are globally unique data... Some examples:
      - Elapsed Time
      - Asset Collections (sounds, textures, meshes)
      - Renderers
*/

use bevy::app::App;
use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

// This is a "system"
fn add_people(mut commands: Commands) {
    // Here we create 3 entities, that have two components attached to them
    // (the Person and the Name components)
    commands.spawn((Person, Name("Elaina Poctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// This is also a "system" that uses the Time and GreetTimer resources
// It also expects a query parameter... we use this below and
// iterate over Name component for entities that also have a Person component
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
