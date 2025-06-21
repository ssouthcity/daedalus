mod pause;
mod settings;

use crate::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<MenuState>();

    app.add_plugins((pause::plugin, settings::plugin));

    app.add_systems(Update, pause_game.run_if(state_changed::<MenuState>));
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    None,
    Pause,
    Settings,
}

fn pause_game(menu_state: Res<State<MenuState>>, mut next_paused: ResMut<NextState<Paused>>) {
    next_paused.set(Paused(*menu_state != MenuState::None))
}
