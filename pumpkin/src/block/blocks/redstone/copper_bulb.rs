use crate::block::blocks::redstone::block_receives_redstone_power;
use crate::block::pumpkin_block::{BlockMetadata, OnPlaceArgs, PumpkinBlock};
use crate::world::World;
use async_trait::async_trait;
use pumpkin_data::Block;
use pumpkin_data::block_properties::BlockProperties;
use pumpkin_data::sound::{Sound, SoundCategory};
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::BlockStateId;
use pumpkin_world::world::BlockFlags;
use std::sync::Arc;

type CopperBulbLikeProperties = pumpkin_data::block_properties::CopperBulbLikeProperties;

pub struct CopperBulbBlock;

impl BlockMetadata for CopperBulbBlock {
    fn namespace(&self) -> &'static str {
        "minecraft"
    }

    fn ids(&self) -> &'static [&'static str] {
        &[
            "copper_bulb",
            "exposed_copper_bulb",
            "weathered_copper_bulb",
            "oxidized_copper_bulb",
            "waxed_copper_bulb",
            "waxed_exposed_copper_bulb",
            "waxed_weathered_copper_bulb",
            "waxed_oxidized_copper_bulb",
        ]
    }
}

#[async_trait]
impl PumpkinBlock for CopperBulbBlock {
    async fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = CopperBulbLikeProperties::default(args.block);
        let is_receiving_power = block_receives_redstone_power(args.world, args.location).await;
        if is_receiving_power {
            props.lit = true;
            args.world
                .play_block_sound(
                    Sound::BlockCopperBulbTurnOn,
                    SoundCategory::Blocks,
                    *args.location,
                )
                .await;
            props.powered = true;
        }
        props.to_state_id(args.block)
    }

    async fn on_neighbor_update(
        &self,
        world: &Arc<World>,
        block: &Block,
        block_pos: &BlockPos,
        _source_block: &Block,
        _notify: bool,
    ) {
        let state = world.get_block_state(block_pos).await;
        let mut props = CopperBulbLikeProperties::from_state_id(state.id, block);
        let is_receiving_power = block_receives_redstone_power(world, block_pos).await;
        if props.powered != is_receiving_power {
            if !props.powered {
                props.lit = !props.lit;
                world
                    .play_block_sound(
                        if props.lit {
                            Sound::BlockCopperBulbTurnOn
                        } else {
                            Sound::BlockCopperBulbTurnOff
                        },
                        SoundCategory::Blocks,
                        *block_pos,
                    )
                    .await;
            }
            props.powered = is_receiving_power;
            world
                .set_block_state(block_pos, props.to_state_id(block), BlockFlags::NOTIFY_ALL)
                .await;
        }
    }
}
