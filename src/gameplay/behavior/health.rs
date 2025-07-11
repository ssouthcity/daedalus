use crate::{assets::LoadResource, pause::PauseableSystems};
use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<HealthAssets>();

    app.register_type::<Health>();

    app.add_event::<Heal>();
    app.add_event::<Hurt>();

    app.add_event::<HealthRestored>();
    app.add_event::<Death>();

    app.add_observer(heal);
    app.add_observer(hurt);

    // Health Bar
    app.register_type::<HealthBar>();
    app.register_type::<HealthBarOf>();

    app.add_observer(spawn_health_bar);
    app.add_systems(Update, update_health_bar_fill.in_set(PauseableSystems));
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
struct HealthAssets {
    #[dependency]
    health_bar: Handle<Image>,
}

impl FromWorld for HealthAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            health_bar: asset_server.load("health_bar.png"),
        }
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
#[relationship_target(relationship= HealthBarOf)]
pub struct HealthBar(Entity);

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
#[relationship(relationship_target = HealthBar)]
pub struct HealthBarOf(pub Entity);

fn spawn_health_bar(
    trigger: Trigger<OnInsert, Health>,
    mut commands: Commands,
    assets: Res<HealthAssets>,
) {
    commands.spawn((
        Name::new("Health Bar"),
        ChildOf(trigger.target()),
        Transform::from_xyz(0.0, 12.0, 0.0),
        Sprite {
            image: assets.health_bar.clone(),
            rect: Rect::new(0.0, 6.0, 16.0, 12.0).into(),
            ..default()
        },
        children![(
            Name::new("Health Bar Fill"),
            HealthBarOf(trigger.target()),
            Transform::from_xyz(-8.0, 0.0, 0.0),
            Sprite {
                image: assets.health_bar.clone(),
                rect: Rect::new(0.0, 0.0, 16.0, 6.0).into(),
                anchor: Anchor::CenterLeft,
                ..default()
            },
        )],
    ));
}

fn update_health_bar_fill(
    healths: Query<(&Health, &HealthBar), Changed<Health>>,
    mut sprites: Query<&mut Sprite>,
) {
    for (health, health_bar) in healths {
        if let Ok(mut sprite) = sprites.get_mut(**health_bar) {
            sprite.custom_size = Some(Vec2::new(health.percentage() * 16.0, 6.0));
        }
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct Heal(pub i32);

#[derive(Event, Deref, DerefMut)]
pub struct Hurt(pub i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub struct Health {
    current: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn amount(&self) -> i32 {
        self.current
    }

    pub fn percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn hurt(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }
}

impl From<&EntityInstance> for Health {
    fn from(entity_instance: &EntityInstance) -> Self {
        let amount = *entity_instance
            .get_int_field("Health")
            .expect("expected entity to have a non-nullable health int field");

        Self::new(amount)
    }
}

#[derive(Event, Reflect)]
pub struct Death;

fn hurt(trigger: Trigger<Hurt>, mut commands: Commands, mut healths: Query<&mut Health>) {
    let Ok(mut health) = healths.get_mut(trigger.target()) else {
        return;
    };

    health.hurt(**trigger.event());

    if health.amount() <= 0 {
        commands.trigger_targets(Death, trigger.target());
        commands.entity(trigger.target()).despawn();
    }
}

#[derive(Event, Reflect, Deref, DerefMut)]
pub struct HealthRestored(pub Entity);

fn heal(
    trigger: Trigger<Heal>,
    mut healths: Query<&mut Health>,
    mut restored_events: EventWriter<HealthRestored>,
) {
    let Ok(mut health) = healths.get_mut(trigger.target()) else {
        return;
    };

    let was_full = health.is_full();

    health.heal(**trigger.event());

    if !was_full && health.is_full() {
        restored_events.write(HealthRestored(trigger.target()));
    }
}
