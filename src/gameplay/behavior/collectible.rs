use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Collector>();
    app.register_type::<Collectible>();
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Collector;

#[derive(Component, Reflect, Copy, Clone, Eq, PartialEq, Debug, Default)]
#[reflect(Component)]
pub struct Collectible;
