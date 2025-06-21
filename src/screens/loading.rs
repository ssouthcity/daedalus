use std::time::Duration;

use crate::prelude::*;
use bevy::prelude::*;
use bevy::time::common_conditions::once_after_delay;

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        enter_gameplay_screen
            .run_if(in_state(Screen::Loading).and(once_after_delay(Duration::from_secs(2)))),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widgets::root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![widgets::header("Loading...")],
    ));
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}
