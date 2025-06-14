use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Daedalus".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
    );

    app.insert_resource(ClearColor(BACKGROUND_COLOR));
}
