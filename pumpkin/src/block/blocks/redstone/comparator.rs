use std::sync::Arc;

use async_trait::async_trait;
use pumpkin_data::{
    Block, BlockDirection, BlockState, HorizontalFacingExt,
    block_properties::{
        BlockProperties, ComparatorLikeProperties, ComparatorMode, EnumVariants, HorizontalFacing,
        RedstoneWireLikeProperties, RepeaterLikeProperties, get_state_by_state_id,
    },
    entity::EntityType,
    item::Item,
};
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::{boundingbox::BoundingBox, position::BlockPos};
use pumpkin_world::{
    BlockStateId,
    block::entities::{BlockEntity, comparator::ComparatorBlockEntity},
    chunk::TickPriority,
    world::{BlockAccessor, BlockFlags},
};

use crate::{
    block::{BlockIsReplacing, pumpkin_block::PumpkinBlock, registry::BlockActionResult},
    entity::player::Player,
    server::Server,
    world::World,
};

use super::get_redstone_power;

#[pumpkin_block("minecraft:comparator")]
pub struct ComparatorBlock;

#[async_trait]
impl PumpkinBlock for ComparatorBlock {
    async fn on_place(
        &self,
        _server: &Server,
        _world: &World,
        player: &Player,
        block: &Block,
        _block_pos: &BlockPos,
        _face: BlockDirection,
        _replacing: BlockIsReplacing,
        _use_item_on: &SUseItemOn,
    ) -> BlockStateId {
        let mut props = ComparatorLikeProperties::default(block);
        let dir = player
            .living_entity
            .entity
            .get_horizontal_facing()
            .opposite();
        props.facing = dir;

        props.to_state_id(block)
    }

    async fn normal_use(
        &self,
        block: &Block,
        _player: &Player,
        location: BlockPos,
        _server: &Server,
        world: &Arc<World>,
    ) {
        let state = world.get_block_state(&location).await;
        let props = ComparatorLikeProperties::from_state_id(state.id, block);
        on_use(props, world, location, block).await;
    }

    async fn use_with_item(
        &self,
        block: &Block,
        _player: &Player,
        location: BlockPos,
        _item: &Item,
        _server: &Server,
        world: &Arc<World>,
    ) -> BlockActionResult {
        let state = world.get_block_state(&location).await;
        let props = ComparatorLikeProperties::from_state_id(state.id, block);
        on_use(props, world, location, block).await;
        BlockActionResult::Consume
    }

    async fn emits_redstone_power(
        &self,
        _block: &Block,
        _state: &BlockState,
        _direction: BlockDirection,
    ) -> bool {
        true
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
        can_place_at(block_accessor, *block_pos).await
    }

    async fn placed(
        &self,
        world: &Arc<World>,
        block: &Block,
        state_id: BlockStateId,
        pos: &BlockPos,
        _old_state_id: BlockStateId,
        _notify: bool,
    ) {
        let comparator = ComparatorBlockEntity::new(*pos);
        world.add_block_entity(Arc::new(comparator)).await;
        if let Some(state) = get_state_by_state_id(state_id) {
            update_target(world, *pos, state.id, block).await;
        }
    }

    async fn player_placed(
        &self,
        world: &Arc<World>,
        block: &Block,
        state_id: u16,
        pos: &BlockPos,
        _face: BlockDirection,
        _player: &Player,
    ) {
        if let Some(state) = get_state_by_state_id(state_id) {
            if has_power(world, *pos, &state, block).await {
                update_target(world, *pos, state.id, block).await;
            }
        }
    }

    async fn broken(
        &self,
        _block: &Block,
        _player: &Arc<Player>,
        block_pos: BlockPos,
        _server: &Server,
        world: Arc<World>,
        _state: BlockState,
    ) {
        world.remove_block_entity(&block_pos).await;
    }

    async fn get_state_for_neighbor_update(
        &self,
        world: &World,
        _block: &Block,
        state: BlockStateId,
        _pos: &BlockPos,
        direction: BlockDirection,
        neighbor_pos: &BlockPos,
        neighbor_state_id: BlockStateId,
    ) -> BlockStateId {
        if direction == BlockDirection::Down {
            if let Some(neighbor_state) = get_state_by_state_id(neighbor_state_id) {
                if can_place_above(world, *neighbor_pos, &neighbor_state) {
                    return Block::AIR.default_state_id;
                }
            }
        }
        state
    }

    async fn get_weak_redstone_power(
        &self,
        block: &Block,
        world: &World,
        block_pos: &BlockPos,
        state: &BlockState,
        direction: BlockDirection,
    ) -> u8 {
        let props = ComparatorLikeProperties::from_state_id(state.id, block);
        if props.powered && props.facing.to_block_direction() == direction {
            get_output_level(world, *block_pos).await
        } else {
            0
        }
    }

    async fn get_strong_redstone_power(
        &self,
        block: &Block,
        world: &World,
        block_pos: &BlockPos,
        state: &BlockState,
        direction: BlockDirection,
    ) -> u8 {
        self.get_weak_redstone_power(block, world, block_pos, state, direction)
            .await
    }

    async fn on_neighbor_update(
        &self,
        world: &Arc<World>,
        block: &Block,
        pos: &BlockPos,
        source_block: &Block,
        _notify: bool,
    ) {
        let state = world.get_block_state(pos).await;
        if can_place_at(&**world, *pos).await {
            update_powered(world, *pos, &state, block).await;
            return;
        }
        world
            .set_block_state(pos, Block::AIR.default_state_id, BlockFlags::NOTIFY_ALL)
            .await;
        for dir in BlockDirection::all() {
            world
                .update_neighbor(&pos.offset(dir.to_offset()), source_block)
                .await;
        }
    }

    async fn on_scheduled_tick(&self, world: &Arc<World>, block: &Block, pos: &BlockPos) {
        let state = world.get_block_state(pos).await;
        update(world, *pos, &state, block).await;
    }

    async fn on_state_replaced(
        &self,
        world: &Arc<World>,
        block: &Block,
        location: BlockPos,
        old_state_id: BlockStateId,
        moved: bool,
    ) {
        if moved || Block::from_state_id(old_state_id).is_some_and(|old_block| old_block == *block)
        {
            return;
        }
        if let Some(old_state) = get_state_by_state_id(old_state_id) {
            update_target(world, location, old_state.id, block).await;
        }
    }
}

async fn on_use(
    mut props: ComparatorLikeProperties,
    world: &Arc<World>,
    block_pos: BlockPos,
    block: &Block,
) {
    props.mode = match props.mode {
        ComparatorMode::Compare => ComparatorMode::Subtract,
        ComparatorMode::Subtract => ComparatorMode::Compare,
    };
    let state_id = props.to_state_id(block);
    world
        .set_block_state(&block_pos, state_id, BlockFlags::empty())
        .await;
    if let Some(state) = get_state_by_state_id(state_id) {
        update(world, block_pos, &state, block).await;
    }
}

async fn can_place_at(world: &dyn BlockAccessor, pos: BlockPos) -> bool {
    let under_pos = pos.down();
    let under_state = world.get_block_state(&under_pos).await;
    can_place_above(world, under_pos, &under_state)
}

fn can_place_above(_world: &dyn BlockAccessor, _pos: BlockPos, state: &BlockState) -> bool {
    state.is_side_solid(BlockDirection::Up)
}

async fn get_output_level(world: &World, pos: BlockPos) -> u8 {
    if let Some((nbt, raw_blockentity)) = world.get_block_entity(&pos).await {
        let comparator = ComparatorBlockEntity::from_nbt(&nbt, pos);
        if raw_blockentity.identifier() == comparator.identifier() {
            return comparator.output_signal as u8;
        }
    }
    0
}

async fn calculate_output_signal(
    world: &World,
    pos: BlockPos,
    state: &BlockState,
    block: &Block,
) -> u8 {
    let power = get_power(world, pos, state, block).await;
    let sub_power = get_max_input_level_sides(world, pos, state, block).await;
    if sub_power >= power {
        return 0;
    }
    let props = ComparatorLikeProperties::from_state_id(state.id, block);
    if props.mode == ComparatorMode::Subtract {
        power - sub_power
    } else {
        power
    }
}

async fn get_max_input_level_sides(
    world: &World,
    pos: BlockPos,
    state: &BlockState,
    block: &Block,
) -> u8 {
    let props = ComparatorLikeProperties::from_state_id(state.id, block);
    let facing = props.facing;

    let power_left = get_power_on_side(world, &pos, facing.rotate_clockwise()).await;
    let power_right = get_power_on_side(world, &pos, facing.rotate_counter_clockwise()).await;

    std::cmp::max(power_left, power_right)
}

async fn get_power_on_side(world: &World, pos: &BlockPos, side: HorizontalFacing) -> u8 {
    let side_pos = pos.offset(side.to_block_direction().to_offset());
    let (side_block, side_state) = world.get_block_and_block_state(&side_pos).await;
    world
        .block_registry
        .get_weak_redstone_power(
            &side_block,
            world,
            &side_pos,
            &side_state,
            side.to_block_direction(),
        )
        .await
}

async fn has_power(world: &World, pos: BlockPos, state: &BlockState, block: &Block) -> bool {
    let i = get_power(world, pos, state, block).await;
    if i == 0 {
        return false;
    }
    let j = get_max_input_level_sides(world, pos, state, block).await;
    if i > j {
        true
    } else {
        let props = ComparatorLikeProperties::from_state_id(state.id, block);
        i == j && props.mode == ComparatorMode::Compare
    }
}

#[allow(dead_code)]
async fn super_has_power(world: &World, pos: BlockPos, state: &BlockState, block: &Block) -> bool {
    get_power(world, pos, state, block).await > 0
}

async fn get_power(world: &World, pos: BlockPos, state: &BlockState, block: &Block) -> u8 {
    let redstone_level = super_get_power(world, pos, state.id, block).await;

    let props = ComparatorLikeProperties::from_state_id(state.id, block);
    let facing = props.facing;
    let source_pos = pos.offset(facing.to_offset());
    let (source_block, source_state) = world.get_block_and_block_state(&source_pos).await;

    if let Some(pumpkin_block) = world.block_registry.get_pumpkin_block(&source_block) {
        if let Some(level) = pumpkin_block
            .get_comparator_output(&source_block, world, &source_pos, &source_state)
            .await
        {
            return level;
        }
    }

    if redstone_level < 15 && source_state.is_solid() {
        let source_pos = source_pos.offset(facing.to_offset());
        let (source_block, source_state) = world.get_block_and_block_state(&source_pos).await;

        let itemframe_level = get_attached_itemframe_level(world, facing, source_pos).await;
        let block_level =
            if let Some(pumpkin_block) = world.block_registry.get_pumpkin_block(&source_block) {
                pumpkin_block
                    .get_comparator_output(&source_block, world, &source_pos, &source_state)
                    .await
            } else {
                None
            };
        if let Some(level) = itemframe_level.max(block_level) {
            return level;
        }
    }
    redstone_level
}

async fn get_attached_itemframe_level(
    world: &World,
    facing: HorizontalFacing,
    pos: BlockPos,
) -> Option<u8> {
    let mut itemframes = world
        .get_entities_at_box(&BoundingBox::from_block(&pos))
        .await
        .into_iter()
        .filter(|entity| {
            entity.get_entity().entity_type == EntityType::ITEM_FRAME
                && entity.get_entity().get_horizontal_facing() == facing
        });
    if let Some(_itemframe) = itemframes.next() {
        if itemframes.next().is_none() {
            // TODO itemframe.getComparatorPower()
            return Some(1);
        }
    }
    None
}

async fn super_get_power(
    world: &World,
    pos: BlockPos,
    state_id: BlockStateId,
    block: &Block,
) -> u8 {
    let props = ComparatorLikeProperties::from_state_id(state_id, block);
    let facing = props.facing;
    let source_pos = pos.offset(facing.to_offset());
    let (source_block, source_state) = world.get_block_and_block_state(&source_pos).await;
    let source_level = get_redstone_power(
        &source_block,
        &source_state,
        world,
        &source_pos,
        facing.to_block_direction(),
    )
    .await;
    if source_level >= 15 {
        source_level
    } else {
        source_level.max(if source_block == Block::REDSTONE_WIRE {
            let props = RedstoneWireLikeProperties::from_state_id(source_state.id, &source_block);
            props.power.to_index() as u8
        } else {
            0
        })
    }
}

async fn update_powered(world: &World, pos: BlockPos, state: &BlockState, block: &Block) {
    if world.is_block_tick_scheduled(&pos, block).await {
        return;
    }
    let i = calculate_output_signal(world, pos, state, block).await;
    let j = get_output_level(world, pos).await;

    let props = ComparatorLikeProperties::from_state_id(state.id, block);
    if i != j || props.powered != has_power(world, pos, state, block).await {
        world
            .schedule_block_tick(
                block,
                pos,
                get_update_delay_internal(state, block),
                if is_target_not_aligned(world, pos, state, block).await {
                    TickPriority::High
                } else {
                    TickPriority::Normal
                },
            )
            .await;
    }
}

async fn is_target_not_aligned(
    world: &dyn BlockAccessor,
    pos: BlockPos,
    state: &BlockState,
    block: &Block,
) -> bool {
    let props = ComparatorLikeProperties::from_state_id(state.id, block);
    let facing = props.facing.opposite();
    let (target_block, target_state) = world
        .get_block_and_block_state(&pos.offset(facing.to_offset()))
        .await;
    if target_block == Block::COMPARATOR {
        let props = ComparatorLikeProperties::from_state_id(target_state.id, &target_block);
        props.facing != facing
    } else if target_block == Block::REPEATER {
        let props = RepeaterLikeProperties::from_state_id(target_state.id, &target_block);
        props.facing != facing
    } else {
        false
    }
}

fn get_update_delay_internal(_state: &BlockState, _block: &Block) -> u16 {
    2
}

async fn update(world: &Arc<World>, pos: BlockPos, state: &BlockState, block: &Block) {
    let i = i32::from(calculate_output_signal(world, pos, state, block).await);
    let lv = world.get_block_entity(&pos).await;
    let mut j = 0;
    if let Some((nbt, blockentity)) = lv {
        if blockentity.identifier() == ComparatorBlockEntity::ID {
            let mut comparator = ComparatorBlockEntity::from_nbt(&nbt, pos);
            j = comparator.output_signal;
            comparator.output_signal = i;
            world.add_block_entity(Arc::new(comparator)).await;
        }
    }
    let mut props = ComparatorLikeProperties::from_state_id(state.id, block);
    if j != i || props.mode == ComparatorMode::Compare {
        let bl = has_power(world, pos, state, block).await;
        let bl2 = props.powered;
        if bl2 && !bl {
            props.powered = false;
            world
                .set_block_state(&pos, props.to_state_id(block), BlockFlags::NOTIFY_LISTENERS)
                .await;
        } else if !bl2 && bl {
            props.powered = true;
            world
                .set_block_state(&pos, props.to_state_id(block), BlockFlags::NOTIFY_LISTENERS)
                .await;
        }
        update_target(world, pos, props.to_state_id(block), block).await;
    }
}

async fn update_target(world: &Arc<World>, pos: BlockPos, state_id: BlockStateId, block: &Block) {
    let props = ComparatorLikeProperties::from_state_id(state_id, block);
    let facing = props.facing;
    let front_pos = pos.offset(facing.opposite().to_offset());
    world.update_neighbor(&front_pos, block).await;
    world
        .update_neighbors(&front_pos, Some(facing.to_block_direction()))
        .await;
}
