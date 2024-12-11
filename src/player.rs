use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(Vec2);

const TIME_STEP: f32 = 1.0 / 60.0;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            image: asset_server.load("player.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(0.1)),
        Velocity(Vec2::ZERO),
    ));
}

pub fn player_movement(
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