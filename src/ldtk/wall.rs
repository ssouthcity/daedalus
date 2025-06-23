use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<WallBundle>(1);

    app.add_systems(Update, process_wall);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct Wall;

#[derive(Clone, Debug, Bundle, Default, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

fn process_wall(mut commands: Commands, query: Query<Entity, Added<Wall>>) {
    for entity in query {
        commands.entity(entity).insert((
            RigidBody::Static,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(16.0, 16.0),
        ));
    }
}
