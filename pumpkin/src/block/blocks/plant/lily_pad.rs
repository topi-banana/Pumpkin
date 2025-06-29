use async_trait::async_trait;
use pumpkin_data::{Block, BlockDirection};
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::world::{BlockAccessor, BlockFlags};

use crate::{
    block::pumpkin_block::{OnEntityCollisionArgs, PumpkinBlock},
    entity::player::Player,
    server::Server,
    world::World,
};

#[pumpkin_block("minecraft:lily_pad")]
pub struct LilyPadBlock;

#[async_trait]
impl PumpkinBlock for LilyPadBlock {
    async fn on_entity_collision(&self, args: OnEntityCollisionArgs<'_>) {
        // Proberbly not the best solution, but works
        if args
            .entity
            .get_entity()
            .entity_type
            .resource_name
            .ends_with("_boat")
        {
            args.world
                .break_block(args.location, None, BlockFlags::empty())
                .await;
        }
    }

    async fn can_place_at(
        &self,
        _server: Option<&Server>,
        _world: Option<&World>,
        block_accessor: &dyn BlockAccessor,
        _player: Option<&Player>,
        _block: &Block,
        block_pos: &BlockPos,
        _face: BlockDirection,
        _use_item_on: Option<&SUseItemOn>,
    ) -> bool {
        let block_below = block_accessor.get_block(&block_pos.down()).await;
        block_below == Block::WATER || block_below == Block::ICE
    }
}
