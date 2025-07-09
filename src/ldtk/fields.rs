use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{health::Health, patrol::Patrol};

impl From<&EntityInstance> for Health {
    fn from(entity_instance: &EntityInstance) -> Self {
        let amount = *entity_instance
            .get_int_field("Health")
            .expect("expected entity to have a non-nullable health int field");

        Self::new(amount)
    }
}

impl From<&EntityInstance> for Patrol {
    fn from(value: &EntityInstance) -> Self {
        let points: VecDeque<Vec2> = value
            .iter_points_field("Patrol")
            .expect("expected entity to have a non-nullable patrol points field")
            .chain(std::iter::once(&value.grid))
            .map(|point| IVec2::new(point.x, 15 - point.y))
            .map(|point| point.as_vec2() * 16.0)
            .collect();

        Self::new(points)
    }
}
