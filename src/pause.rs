use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Paused>();
    app.configure_sets(Update, PauseableSystems.run_if(in_state(Paused(false))));
}

#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[states(scoped_entities)]
pub struct Paused(pub bool);

#[derive(SystemSet, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct PauseableSystems;
