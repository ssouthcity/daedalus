use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    collectible::{Collectible, Potion},
    health::Health,
};

#[derive(Clone, Default, Debug, Bundle, LdtkEntity)]
pub struct PotionEntity {
    pub collectible: Collectible,
    pub potion: Potion,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[with(Health::from_ldtk_field)]
    pub health: Health,
}
