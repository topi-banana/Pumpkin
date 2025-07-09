use async_trait::async_trait;
use pumpkin_data::entity::EffectType;
use pumpkin_nbt::compound::NbtCompound;
use pumpkin_util::math::position::BlockPos;

use super::BlockEntity;

pub struct BeaconBlockEntity {
    pub position: BlockPos,
    pub level: i32,
    pub primary_effect: Option<EffectType>,
    pub secondary_effect: Option<EffectType>,
}

impl BeaconBlockEntity {
    pub const ID: &'static str = "minecraft:beacon";
    pub fn new(position: BlockPos) -> Self {
        Self {
            position,
            level: 0,
            primary_effect: None,
            secondary_effect: None,
        }
    }
}

#[async_trait]
impl BlockEntity for BeaconBlockEntity {
    fn resource_location(&self) -> &'static str {
        Self::ID
    }

    fn get_position(&self) -> BlockPos {
        self.position
    }

    fn from_nbt(nbt: &NbtCompound, position: BlockPos) -> Self
    where
        Self: Sized,
    {
        Self {
            position,
            level: nbt.get_int("Levels").unwrap_or(0),
            primary_effect: nbt
                .get_string("primary_effect")
                .and_then(|effect| EffectType::from_name(effect)),
            secondary_effect: nbt
                .get_string("secondary_effect")
                .and_then(|effect| EffectType::from_name(effect)),
        }
    }

    async fn write_nbt(&self, nbt: &mut NbtCompound) {
        nbt.put_int("Levels", self.level);
        if let Some(effect) = self.primary_effect {
            nbt.put_string("primary_effect", effect.to_name().to_string());
        }
        if let Some(effect) = self.secondary_effect {
            nbt.put_string("secondary_effect", effect.to_name().to_string());
        }
    }

    fn chunk_data_nbt(&self) -> Option<NbtCompound> {
        let mut nbt = NbtCompound::new();
        nbt.put_int("Levels", self.level);
        if let Some(effect) = self.primary_effect {
            nbt.put_string("primary_effect", effect.to_name().to_string());
        }
        if let Some(effect) = self.secondary_effect {
            nbt.put_string("secondary_effect", effect.to_name().to_string());
        }
        Some(nbt)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
