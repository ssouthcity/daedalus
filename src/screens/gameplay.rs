use crate::{gameplay::spawn_world, menus::Menu, pause::Paused, screens::Screen};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_world);

    app.add_systems(
        Update,
        open_menu.run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(OnExit(Menu::None), pause.run_if(in_state(Screen::Gameplay)));

    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn open_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn pause(mut next_pause: ResMut<NextState<Paused>>) {
    next_pause.set(Paused(true));
}

fn unpause(mut next_pause: ResMut<NextState<Paused>>) {
    next_pause.set(Paused(false));
}
