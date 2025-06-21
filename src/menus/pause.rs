use crate::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::MenuState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Pause), spawn_pause_menu);

    app.add_systems(
        Update,
        pause.run_if(in_state(MenuState::None).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(
        Update,
        unpause.run_if(in_state(MenuState::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widgets::root("Pause menu"),
        widgets::background_dimmed(),
        StateScoped(MenuState::Pause),
        children![(
            widgets::card(),
            children![
                widgets::header("Paused"),
                (
                    widgets::list(),
                    children![
                        widgets::button("Resume", resume),
                        widgets::button("Settings", settings),
                        widgets::button("Quit", quit),
                    ]
                )
            ],
        )],
    ));
}

fn pause(mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::Pause);
}

fn unpause(mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::None);
}

fn resume(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::None);
}

fn settings(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::Settings);
}

fn quit(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
