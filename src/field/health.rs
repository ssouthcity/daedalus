use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub struct Health(pub i32);

impl Health {
    pub fn from_field(entity_instance: &EntityInstance) -> Health {
        let amount = *entity_instance
            .get_int_field("Health")
            .expect("expected entity to have a non-nullable health int field");

        Health(amount)
    }
}
