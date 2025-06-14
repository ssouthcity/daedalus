use bevy::prelude::*;

use crate::{health::Health, player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_hud_system)
        .add_systems(Update, update_hp_system);
}

#[derive(Component)]
struct HudHPText;

fn setup_hud_system(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(32.0), Val::Px(24.0)),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn((HudHPText, Text::default()));
        });
}

fn update_hp_system(
    mut hud_hp_text: Single<&mut Text, With<HudHPText>>,
    hp: Single<&Health, With<player::Player>>,
) {
    hud_hp_text.0 = hp.amount().to_string();
}
