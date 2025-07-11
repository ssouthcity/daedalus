use bevy::prelude::*;

use crate::{
    gameplay::{
        behavior::{
            health::Health,
            inventory::{InventoryIcon, ItemOf},
        },
        movement::Player,
    },
    pause::PauseableSystems,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_hud_system);

    app.add_systems(Update, update_hp_system.in_set(PauseableSystems));

    app.add_observer(add_item_to_inventory);
    app.add_observer(remove_item_from_inventory);
}

#[derive(Component)]
struct HudHPText;

#[derive(Component)]
struct InventoryUI;

#[derive(Component, Deref, DerefMut)]
struct ItemSlotOf(Entity);

fn setup_hud_system(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(32.0), Val::Px(24.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|commands| {
            commands.spawn((HudHPText, Text::default()));

            commands.spawn((
                InventoryUI,
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(64.0),
                    ..default()
                },
            ));

            commands.spawn(Node::default());
        });
}

fn update_hp_system(
    mut hud_hp_text: Single<&mut Text, With<HudHPText>>,
    hp: Single<&Health, With<Player>>,
) {
    hud_hp_text.0 = hp.amount().to_string();
}

fn add_item_to_inventory(
    trigger: Trigger<OnInsert, ItemOf>,
    mut commands: Commands,
    inventory_ui_root: Single<Entity, With<InventoryUI>>,
    icons: Query<&InventoryIcon>,
) -> Result {
    let icon = icons.get(trigger.target())?;

    commands.spawn((
        Name::new("Item"),
        ChildOf(*inventory_ui_root),
        ItemSlotOf(trigger.target()),
        ImageNode {
            image: icon.0.clone(),
            ..default()
        },
    ));

    Ok(())
}

fn remove_item_from_inventory(
    trigger: Trigger<OnRemove, ItemOf>,
    mut commands: Commands,
    item_slots: Query<(Entity, &ItemSlotOf)>,
) {
    for (entity, item_slot_of) in item_slots {
        if **item_slot_of == trigger.target() {
            commands.entity(entity).despawn();
        }
    }
}
