use bevy::prelude::*;

pub mod collectible;
pub mod health;
pub mod inventory;
pub mod patrol;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        collectible::plugin,
        health::plugin,
        inventory::plugin,
        patrol::plugin,
    ));
}
