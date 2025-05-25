use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn move_player(
    mut query: Query<&mut LinearVelocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut axis = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        axis += Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        axis -= Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        axis -= Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        axis += Vec2::X;
    }

    axis = axis.normalize();

    for mut velocity in query.iter_mut() {
        velocity.0 = axis * 128.0;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), Vec2::splat(32.0)),
        // Physics
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(32.0, 32.0),
        LinearVelocity(Vec2::ZERO),
        AngularVelocity::ZERO,
        Friction::ZERO,
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 200.0, 0.0),
        Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(600.0, 32.0)),
        RigidBody::Static,
        Collider::rectangle(600.0, 32.0),
        Friction::ZERO,
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 1.0)))
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}
