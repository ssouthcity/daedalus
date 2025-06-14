use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::UiDebugOptions};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        WorldInspectorPlugin::default().run_if(dev_tools_enabled),
        PhysicsDebugPlugin::default(),
    ));

    app.init_resource::<DevToolsEnabled>();

    app.add_systems(
        Update,
        toggle_dev_tools_enabled.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    app.add_systems(
        Update,
        (toggle_debug_ui, toggle_avian_gizmos).run_if(resource_changed::<DevToolsEnabled>),
    );
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct DevToolsEnabled(pub bool);

fn dev_tools_enabled(dev_tools_enabled: Res<DevToolsEnabled>) -> bool {
    dev_tools_enabled.0
}

fn toggle_dev_tools_enabled(mut dev_tools_enabled: ResMut<DevToolsEnabled>) {
    dev_tools_enabled.0 = !dev_tools_enabled.0;
}

fn toggle_debug_ui(dev_tools_enabled: Res<DevToolsEnabled>, mut options: ResMut<UiDebugOptions>) {
    options.enabled = dev_tools_enabled.0;
}

fn toggle_avian_gizmos(
    dev_tools_enabled: Res<DevToolsEnabled>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
) {
    let (gizmo_config, _) = gizmo_config_store.config_mut::<PhysicsGizmos>();
    gizmo_config.enabled = dev_tools_enabled.0;
}
