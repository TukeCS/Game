use bevy::prelude::*;

mod components;
mod physics;
mod player;
mod systems;

use physics::*;
use player::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(
            RunFixedMainLoop,
            (
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        )
        .run();
}
