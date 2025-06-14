use bevy_ecs_ldtk::prelude::*;

use crate::health::Health;

impl Health {
    pub fn from_ldtk_field(entity_instance: &EntityInstance) -> Health {
        let amount = *entity_instance
            .get_int_field("Health")
            .expect("expected entity to have a non-nullable health int field");

        Health::new(amount)
    }
}
