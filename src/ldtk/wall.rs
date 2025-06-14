use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,

    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub friction: Friction,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall::default(),

            rigid_body: RigidBody::Static,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::rectangle(16.0, 16.0),
            linear_velocity: LinearVelocity::default(),
            angular_velocity: AngularVelocity::default(),
            friction: Friction::default(),
        }
    }
}
