use crate::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::menus::MenuState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Settings), spawn_settings_menu);

    app.add_systems(
        Update,
        go_back.run_if(in_state(MenuState::Settings).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widgets::root("Settings menu"),
        widgets::background_dimmed(),
        StateScoped(MenuState::Settings),
        children![(
            widgets::card(),
            children![
                widgets::header("Paused"),
                (
                    widgets::list(),
                    children![
                        widgets::button("Toggle mute", toggle_mute),
                        (
                            Node {
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(16.0),
                                ..default()
                            },
                            children![
                                Text::new("Volume"),
                                widgets::button(" - ", decrease_volume),
                                widgets::button(" + ", increase_volume),
                            ],
                        ),
                        widgets::button("Back", trigger_go_back),
                    ]
                )
            ],
        )],
    ));
}

fn toggle_mute(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.toggle_mute();
}

fn decrease_volume(_: Trigger<Pointer<Click>>, mut master_volume: ResMut<MasterVolume>) {
    master_volume.decrement();
}

fn increase_volume(_: Trigger<Pointer<Click>>, mut master_volume: ResMut<MasterVolume>) {
    master_volume.increment();
}

fn trigger_go_back(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::Pause);
}

fn go_back(mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::Pause);
}
