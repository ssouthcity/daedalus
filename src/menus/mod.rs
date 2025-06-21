mod pause;
mod settings;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.add_plugins((pause::plugin, settings::plugin));
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Pause,
    Settings,
}
