use bevy::prelude::*;

mod ogre;
mod player;
mod potion;
mod wall;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ogre::plugin, player::plugin, potion::plugin, wall::plugin));
}

fn fix_z_coordinate<C: Component>(transforms: Query<&mut Transform, With<C>>) {
    for mut transform in transforms {
        transform.translation.z = 0.0;
    }
}
