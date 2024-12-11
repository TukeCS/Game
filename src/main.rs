use bevy::prelude::*;

mod player;

use player::{spawn_player, player_movement};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Update,player_movement)
        .run();
}