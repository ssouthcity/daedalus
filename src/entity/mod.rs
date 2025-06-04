use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod player;
mod potion;

pub struct LdtkEntityPlugin;

impl Plugin for LdtkEntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<player::PlayerEntity>("Player")
            .register_ldtk_entity::<potion::PotionEntity>("Health_Potion")
            .add_systems(Update, player::process_player);
    }
}
