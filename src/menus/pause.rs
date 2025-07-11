use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::theme::widgets;

use super::Menu;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);

    app.add_systems(
        Update,
        close_menu.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widgets::root("Pause menu"),
        widgets::background_dimmed(),
        StateScoped(Menu::Pause),
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

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn resume(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn settings(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn quit(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
