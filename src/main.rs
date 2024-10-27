use bevy::prelude::*;

const CUBE_COLOR: Color = Color::hsl(139.0, 1.0, 0.54);
const CUBE_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: CUBE_COLOR,
            custom_size: Some(Vec2::splat(CUBE_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    });
}
