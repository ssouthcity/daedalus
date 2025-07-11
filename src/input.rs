use bevy::prelude::*;

use crate::pause::PauseableSystems;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(InputDevice::default());
    app.insert_resource(MovementInput::default());

    app.add_event::<InteractInput>();

    app.add_systems(
        PreUpdate,
        (
            collect_keyboard_axis,
            collect_keyboard_sprinting,
            collect_keyboard_interact,
        )
            .run_if(resource_equals(InputDevice::MouseAndKeyboard))
            .in_set(PauseableSystems),
    );
}

#[derive(Default, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub enum InputDevice {
    #[default]
    MouseAndKeyboard,
    Controller,
}

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct MovementInput {
    pub axis: Vec2,
    pub is_sprinting: bool,
}

#[derive(Default, Event)]
pub struct InteractInput;

fn collect_keyboard_axis(
    input: Res<ButtonInput<KeyCode>>,
    mut movement_input: ResMut<MovementInput>,
) {
    let mut axis = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {
        axis += Vec2::Y;
    }

    if input.pressed(KeyCode::KeyA) {
        axis -= Vec2::X;
    }

    if input.pressed(KeyCode::KeyS) {
        axis -= Vec2::Y;
    }

    if input.pressed(KeyCode::KeyD) {
        axis += Vec2::X;
    }

    movement_input.axis = axis.normalize_or_zero();
}

fn collect_keyboard_sprinting(
    input: Res<ButtonInput<KeyCode>>,
    mut movement_input: ResMut<MovementInput>,
) {
    movement_input.is_sprinting = input.pressed(KeyCode::ShiftLeft);
}

fn collect_keyboard_interact(
    input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<InteractInput>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        events.write_default();
    }
}
