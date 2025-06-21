mod gameplay;
mod loading;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((gameplay::plugin, loading::plugin));
}

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Loading,
    Gameplay,
}
