use bevy::{
    input::common_conditions::{input_just_pressed, input_toggle_active},
    prelude::*,
    ui::UiDebugOptions,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, TOGGLE_KEY)),
        ));

        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
