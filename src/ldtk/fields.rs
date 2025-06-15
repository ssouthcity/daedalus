use bevy_ecs_ldtk::prelude::*;

use crate::health::Health;

impl From<&EntityInstance> for Health {
    fn from(entity_instance: &EntityInstance) -> Self {
        let amount = *entity_instance
            .get_int_field("Health")
            .expect("expected entity to have a non-nullable health int field");

        Self::new(amount)
    }
}
