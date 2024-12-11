use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Update,player_movement)
        .run();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            image: asset_server.load("player.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(4.0)),
        Velocity(Vec2::ZERO),
    ));

}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    const SPEED: f32 = 210.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        velocity.0 = direction.normalize_or_zero() * SPEED;
        transform.translation += velocity.0.extend(0.0) * TIME_STEP;
    }
}

#[derive(Component, Default)]
struct Velocity(Vec2);

const TIME_STEP: f32 = 1.0 / 60.0;
