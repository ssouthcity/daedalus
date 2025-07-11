use bevy::prelude::*;

mod animation;
mod assets;
mod audio;
mod camera;
mod default;
mod dev_tools;
mod gameplay;
mod input;
mod menus;
mod pause;
mod physics;
mod screens;
mod theme;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(default::plugin);

        app.add_plugins((
            animation::plugin,
            assets::plugin,
            audio::plugin,
            camera::plugin,
            gameplay::plugin,
            input::plugin,
            menus::plugin,
            pause::plugin,
            physics::plugin,
            screens::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}
