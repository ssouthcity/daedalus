use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<HealEvent>()
        .add_event::<HurtEvent>()
        .add_event::<KillEvent>()
        .add_event::<RestoredEvent>();

    app.register_type::<Health>();

    app.add_systems(Update, (heal_entities, hurt_entities));
}

#[derive(Event)]
pub struct HealEvent {
    pub entity: Entity,
    pub amount: i32,
}

#[derive(Event)]
pub struct HurtEvent {
    pub entity: Entity,
    pub amount: i32,
}

#[derive(Event)]
pub struct KillEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct RestoredEvent {
    pub entity: Entity,
}

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

    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn is_depleted(&self) -> bool {
        self.current <= 0
    }

    fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    fn hurt(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }
}

fn heal_entities(
    mut events: EventReader<HealEvent>,
    mut restored_events: EventWriter<RestoredEvent>,
    mut health: Query<&mut Health>,
) {
    for event in events.read() {
        if let Ok(mut hp) = health.get_mut(event.entity) {
            if hp.is_full() {
                continue;
            }

            hp.heal(event.amount);

            if hp.is_full() {
                restored_events.write(RestoredEvent {
                    entity: event.entity,
                });
            }
        }
    }
}

fn hurt_entities(
    mut hurt_events: EventReader<HurtEvent>,
    mut kill_events: EventWriter<KillEvent>,
    mut health: Query<&mut Health>,
) {
    for event in hurt_events.read() {
        if let Ok(mut hp) = health.get_mut(event.entity) {
            hp.hurt(event.amount);

            if hp.is_depleted() {
                kill_events.write(KillEvent {
                    entity: event.entity,
                });
            }
        }
    }
}
