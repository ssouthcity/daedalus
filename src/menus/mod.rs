mod pause;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<MenuState>();

    app.add_plugins((pause::plugin,));
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    None,
    Pause,
}
