use bevy::prelude::*;

mod audio;
mod camera;
mod collectible;
mod debug;
mod default;
mod dev_tools;
mod health;
mod hud;
mod input;
mod ldtk;
mod physics;
mod player;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            audio::plugin,
            camera::plugin,
            collectible::plugin,
            default::plugin,
            health::plugin,
            hud::plugin,
            input::plugin,
            ldtk::plugin,
            physics::plugin,
            player::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins((debug::plugin, dev_tools::plugin));
    }
}
