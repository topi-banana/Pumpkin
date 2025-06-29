use async_trait::async_trait;
use pumpkin_data::{
    Block, BlockDirection, HorizontalFacingExt,
    block_properties::{BlockFace, BlockProperties, GrindstoneLikeProperties},
};
use pumpkin_macros::pumpkin_block;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::{BlockStateId, world::BlockAccessor};

use crate::block::pumpkin_block::OnPlaceArgs;
use crate::block::pumpkin_block::PumpkinBlock;
use crate::{block::pumpkin_block::CanPlaceAtArgs, world::World};

use super::abstruct_wall_mounting::WallMountedBlock;

#[pumpkin_block("minecraft:grindstone")]
pub struct GrindstoneBlock;

#[async_trait]
impl PumpkinBlock for GrindstoneBlock {
    async fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props =
            GrindstoneLikeProperties::from_state_id(args.block.default_state.id, args.block);
        (props.face, props.facing) =
            WallMountedBlock::get_placement_face(self, args.player, args.direction);

        props.to_state_id(args.block)
    }

    async fn can_place_at(&self, args: CanPlaceAtArgs<'_>) -> bool {
        WallMountedBlock::can_place_at(self, args.block_accessor, args.location, args.direction)
            .await
    }

    async fn get_state_for_neighbor_update(
        &self,
        world: &World,
        block: &Block,
        state: BlockStateId,
        pos: &BlockPos,
        direction: BlockDirection,
        _neighbor_pos: &BlockPos,
        _neighbor_state: BlockStateId,
    ) -> BlockStateId {
        WallMountedBlock::get_state_for_neighbor_update(self, state, block, direction, world, pos)
            .await
            .unwrap_or(state)
    }
}

#[async_trait]
impl WallMountedBlock for GrindstoneBlock {
    async fn can_place_at(
        &self,
        _world: &dyn BlockAccessor,
        _pos: &BlockPos,
        _direction: &BlockDirection,
    ) -> bool {
        true
    }

    fn get_direction(&self, state_id: BlockStateId, block: &Block) -> BlockDirection {
        let props = GrindstoneLikeProperties::from_state_id(state_id, block);
        match props.face {
            BlockFace::Floor => BlockDirection::Up,
            BlockFace::Ceiling => BlockDirection::Down,
            BlockFace::Wall => props.facing.to_block_direction(),
        }
    }
}
