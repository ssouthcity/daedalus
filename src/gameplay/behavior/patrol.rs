use std::collections::VecDeque;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// how close the entity can be to the patrol point before it is considered as reached
const PATROL_GOAL_EPSILON: f32 = 2.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Patrol>();
    app.register_type::<PatrolSpeed>();

    app.add_systems(
        Update,
        (set_next_patrol_target, move_toward_next_point).chain(),
    );
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
#[require(PatrolSpeed)]
pub struct Patrol(VecDeque<Vec2>);

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PatrolSpeed(f32);

impl Default for PatrolSpeed {
    fn default() -> Self {
        Self(16.0)
    }
}

impl Patrol {
    pub fn new(points: VecDeque<Vec2>) -> Self {
        Self(points)
    }

    pub fn next(&self) -> Option<&Vec2> {
        self.front()
    }

    pub fn shift(&mut self) {
        let Some(front) = self.pop_front() else {
            return;
        };

        self.push_back(front);
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

fn move_toward_next_point(query: Query<(&Transform, &Patrol, &PatrolSpeed, &mut LinearVelocity)>) {
    for (transform, patrol, patrol_speed, mut linear_velocity) in query {
        let Some(goal) = patrol.next() else {
            continue;
        };

        let direction = (goal - transform.translation.truncate()).normalize();

        **linear_velocity = direction * (**patrol_speed);
    }
}

fn set_next_patrol_target(query: Query<(&Transform, &mut Patrol)>) {
    for (transform, mut patrol) in query {
        let Some(goal) = patrol.next() else {
            continue;
        };

        let distance_to_goal = transform
            .translation
            .distance(goal.extend(transform.translation.z));

        if distance_to_goal <= PATROL_GOAL_EPSILON {
            patrol.shift();
        }
    }
}
