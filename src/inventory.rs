use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{health::Health, input::InteractInput, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Inventory>();
    app.register_type::<ItemOf>();
    app.register_type::<InventoryIcon>();

    app.add_event::<OnUseItem>();

    app.add_systems(Update, use_item.run_if(on_event::<InteractInput>));

    app.add_systems(
        Update,
        (|query: Query<&mut Health>| {
            for mut hp in query {
                hp.hurt(1);
            }
        })
        .run_if(on_timer(Duration::from_secs(1))),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship= ItemOf, linked_spawn)]
pub struct Inventory(Vec<Entity>);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[relationship(relationship_target = Inventory)]
pub struct ItemOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InventoryIcon(pub Handle<Image>);

#[derive(Event, Reflect)]
pub struct OnUseItem;

fn use_item(
    mut commands: Commands,
    inventories: Query<&Inventory>,
    player: Single<Entity, With<Player>>,
) {
    if let Some(item) = inventories.iter_descendants(*player).next() {
        commands.trigger_targets(OnUseItem, item);
    }
}
