use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    collectible::{Collectible, Potion},
    field::Health,
};

#[derive(Clone, Default, Debug, Bundle, LdtkEntity)]
pub struct PotionEntity {
    pub collectible: Collectible,
    pub potion: Potion,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[with(Health::from_field)]
    pub health: Health,
}
