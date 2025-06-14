use bevy::prelude::*;

use daedalus::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
