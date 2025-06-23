use bevy::prelude::*;

mod assets;
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
mod menus;
mod pause;
mod physics;
mod player;
mod prelude;
mod screens;
mod theme;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::plugin,
            audio::plugin,
            camera::plugin,
            collectible::plugin,
            default::plugin,
            health::plugin,
            hud::plugin,
            input::plugin,
            ldtk::plugin,
            menus::plugin,
            pause::plugin,
            physics::plugin,
            player::plugin,
            screens::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins((debug::plugin, dev_tools::plugin));
    }
}
