use std::sync::Arc;

use async_trait::async_trait;
use pumpkin_data::{Block, BlockDirection, BlockState};
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::{BlockStateId, world::BlockAccessor};

use crate::{
    block::{BlockIsReplacing, pumpkin_block::PumpkinBlock},
    entity::player::Player,
    server::Server,
    world::World,
};

#[pumpkin_block("minecraft:tripwire_hook")]
pub struct TripwireHookBlock;

#[async_trait]
impl PumpkinBlock for TripwireHookBlock {
    async fn on_place(
        &self,
        _server: &Server,
        _world: &World,
        _player: &Player,
        block: &Block,
        _block_pos: &BlockPos,
        _face: BlockDirection,
        _replacing: BlockIsReplacing,
        _use_item_on: &SUseItemOn,
    ) -> BlockStateId {
        block.default_state_id
    }

    async fn can_place_at(
        &self,
        _server: Option<&Server>,
        _world: Option<&World>,
        _block_accessor: &dyn BlockAccessor,
        _player: Option<&Player>,
        _block: &Block,
        _block_pos: &BlockPos,
        _face: BlockDirection,
        _use_item_on: Option<&SUseItemOn>,
    ) -> bool {
        true
    }

    async fn get_state_for_neighbor_update(
        &self,
        _world: &World,
        _block: &Block,
        state: BlockStateId,
        _pos: &BlockPos,
        _direction: BlockDirection,
        _neighbor_pos: &BlockPos,
        _neighbor_state: BlockStateId,
    ) -> BlockStateId {
        state
    }

    async fn on_scheduled_tick(&self, _world: &Arc<World>, _block: &Block, _pos: &BlockPos) {
    }

    async fn on_state_replaced(
        &self,
        _world: &Arc<World>,
        _block: &Block,
        _location: BlockPos,
        _old_state_id: BlockStateId,
        _moved: bool,
    ) {
    }

    async fn emits_redstone_power(
        &self,
        _block: &Block,
        _state: &BlockState,
        _direction: BlockDirection,
    ) -> bool {
        false
    }

    async fn get_weak_redstone_power(
        &self,
        _block: &Block,
        _world: &World,
        _pos: &BlockPos,
        _state: &BlockState,
        _direction: BlockDirection,
    ) -> u8 {
        0
    }

    async fn get_strong_redstone_power(
        &self,
        _block: &Block,
        _world: &World,
        _pos: &BlockPos,
        _state: &BlockState,
        _direction: BlockDirection,
    ) -> u8 {
        0
    }
}

impl TripwireHookBlock {
    #[allow(clippy::unused_async)]
    pub async fn update(
        _world: &Arc<World>,
        _pos: &BlockPos,
        _state: &BlockState,
        _bl: bool,
        _bl2: bool,
        _i: i32,
        _arg4: Option<&BlockState>,
    ) {
    }
}
