use bevy::prelude::*;

// component
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

// system
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

// system
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Position { x: 16.0, y: 32.0 },
        Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), Vec2::splat(32.0)),
    ));
}

fn translate_position_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

fn move_position_system(mut query: Query<&mut Position>) {
    for mut pos in &mut query {
        pos.x += 1.0;
        pos.y += 1.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_position_system,
                translate_position_system,
                print_position_system,
            )
                .chain(),
        )
        .run();
}
