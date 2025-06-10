use std::sync::Arc;

use async_trait::async_trait;
use pumpkin_data::{
    Block, BlockDirection, BlockState,
    block_properties::{BlockProperties, get_block_by_state_id},
    sound::{Sound, SoundCategory},
};
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::{
    BlockStateId,
    chunk::TickPriority,
    world::{BlockAccessor, BlockFlags},
};
use rand::{Rng, thread_rng};

use crate::{
    block::{BlockIsReplacing, pumpkin_block::PumpkinBlock},
    entity::player::Player,
    server::Server,
    world::World,
};

type TripwireProperties = pumpkin_data::block_properties::TripwireLikeProperties;
type TripwireHookProperties = pumpkin_data::block_properties::TripwireHookLikeProperties;

#[pumpkin_block("minecraft:tripwire_hook")]
pub struct TripwireHookBlock;

#[async_trait]
impl PumpkinBlock for TripwireHookBlock {
    async fn on_place(
        &self,
        _server: &Server,
        world: &World,
        _player: &Player,
        block: &Block,
        block_pos: &BlockPos,
        face: BlockDirection,
        _replacing: BlockIsReplacing,
        _use_item_on: &SUseItemOn,
    ) -> BlockStateId {
        let mut props = TripwireHookProperties::default(block);
        props.powered = false;
        props.attached = false;
        if let Some(facing) = face.to_horizontal_facing() {
            props.facing = facing.opposite();
            if Self::can_place_at(world, block_pos, face).await {
                return props.to_state_id(block);
            }
        }
        block.default_state_id
    }

    async fn can_place_at(
        &self,
        _server: Option<&Server>,
        world: Option<&World>,
        _block_accessor: &dyn BlockAccessor,
        _player: Option<&Player>,
        _block: &Block,
        block_pos: &BlockPos,
        face: BlockDirection,
        _use_item_on: Option<&SUseItemOn>,
    ) -> bool {
        if let Some(world) = world {
            Self::can_place_at(world, block_pos, face).await
        } else {
            false
        }
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
        if direction.to_horizontal_facing().is_some_and(|facing| {
            let props = TripwireHookProperties::from_state_id(state, block);
            facing.opposite() == props.facing
        }) && !Self::can_place_at(world, pos, direction).await
        {
            Block::AIR.default_state_id
        } else {
            state
        }
    }

    async fn on_scheduled_tick(&self, world: &Arc<World>, _block: &Block, pos: &BlockPos) {
        let state_id = world.get_block_state_id(pos).await;
        Self::update(world, *pos, state_id, false, true, -1, None).await;
    }

    async fn on_state_replaced(
        &self,
        world: &Arc<World>,
        block: &Block,
        location: BlockPos,
        old_state_id: BlockStateId,
        moved: bool,
    ) {
        if moved || get_block_by_state_id(old_state_id).is_some_and(|old_block| old_block == *block)
        {
            return;
        }
        let props = TripwireHookProperties::from_state_id(old_state_id, block);
        if props.powered || props.attached {
            Self::update(world, location, old_state_id, true, false, -1, None).await;
        }
        if props.powered {
            world.update_neighbor(&location, block).await;
            world
                .update_neighbor(&location.offset(props.facing.opposite().to_offset()), block)
                .await;
        }
    }

    #[inline]
    async fn emits_redstone_power(
        &self,
        _block: &Block,
        _state: &BlockState,
        _direction: BlockDirection,
    ) -> bool {
        true
    }

    async fn get_weak_redstone_power(
        &self,
        block: &Block,
        _world: &World,
        _pos: &BlockPos,
        state: &BlockState,
        _direction: BlockDirection,
    ) -> u8 {
        let props = TripwireHookProperties::from_state_id(state.id, block);
        if props.powered { 15 } else { 0 }
    }

    async fn get_strong_redstone_power(
        &self,
        block: &Block,
        _world: &World,
        _pos: &BlockPos,
        state: &BlockState,
        direction: BlockDirection,
    ) -> u8 {
        let props = TripwireHookProperties::from_state_id(state.id, block);
        if props.powered
            && direction
                .to_horizontal_facing()
                .is_some_and(|facing| props.facing == facing)
        {
            15
        } else {
            0
        }
    }
}

impl TripwireHookBlock {
    pub async fn can_place_at(world: &World, block_pos: &BlockPos, face: BlockDirection) -> bool {
        if !face.is_horizontal() {
            return false;
        }
        let place_block_pos = block_pos.offset(face.opposite().to_offset());
        let place_block = world.get_block(&place_block_pos).await;
        // TODO isSideSolidFullSquare
        if place_block == Block::AIR {
            return false;
        }
        true
    }

    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    pub async fn update(
        world: &Arc<World>,
        pos: BlockPos,
        state_id: BlockStateId,
        bl: bool,
        bl2: bool,
        i: i32,
        arg4: Option<TripwireProperties>,
    ) {
        let props = TripwireHookProperties::from_state_id(state_id, &Block::TRIPWIRE_HOOK);
        let lv = props.facing;
        let bl3 = props.attached;
        let bl4 = props.powered;
        let lv2 = world.get_block(&pos).await;
        let mut bl5 = !bl;
        let mut n = false;
        let mut j = 0;
        let mut lvs: Vec<Option<TripwireProperties>> = vec![None; 42];

        for k in 1..42 {
            let lv3 = pos.offset_dir(lv.to_offset(), k);
            let lv4 = world.get_block(&lv3).await;
            if lv4 == Block::TRIPWIRE_HOOK {
                let current_props = {
                    let state_id = world.get_block_state_id(&lv3).await;
                    TripwireHookProperties::from_state_id(state_id, &lv4)
                };
                if current_props.facing == lv.opposite() {
                    j = k;
                }
                break;
            }
            if lv4 == Block::TRIPWIRE || k == i {
                let mut current_props = {
                    let state_id = world.get_block_state_id(&lv3).await;
                    TripwireProperties::from_state_id(state_id, &lv4)
                };
                if k == i {
                    current_props = arg4.unwrap_or(current_props);
                }
                let bl7 = !current_props.disarmed;
                let bl8 = current_props.powered;
                n |= bl7 && bl8;
                lvs[k as usize] = Some(current_props);
                if k == i {
                    world
                        .schedule_block_tick(&lv2, pos, 10, TickPriority::Normal)
                        .await;
                    bl5 &= bl7;
                }
            } else {
                lvs[k as usize] = None;
                bl5 = false;
            }
        }

        let m = bl5 & (j > 1);
        n &= m;
        let mut lv5 = TripwireHookProperties::default(&lv2);
        lv5.attached = m;
        lv5.powered = n;

        if j > 0 {
            let lv3 = pos.offset_dir(lv.to_offset(), j);
            let lv6 = lv.opposite();
            let mut lv5_clone = lv5;
            lv5_clone.facing = lv6;
            world
                .set_block_state(
                    &lv3,
                    lv5_clone.to_state_id(&Block::TRIPWIRE_HOOK),
                    BlockFlags::NOTIFY_ALL,
                )
                .await;
            Self::update_neighbors_on_axis(
                &lv2,
                world,
                lv3,
                BlockDirection::from_cardinal_direction(lv6),
            )
            .await;
            Self::play_sound(world, &lv3, m, n, bl3, bl4).await;
        }

        Self::play_sound(world, &pos, m, n, bl3, bl4).await;

        if !bl {
            let mut lv5_clone = lv5;
            lv5_clone.facing = lv;
            world
                .set_block_state(
                    &pos,
                    lv5_clone.to_state_id(&Block::TRIPWIRE_HOOK),
                    BlockFlags::NOTIFY_ALL,
                )
                .await;
            if bl2 {
                Self::update_neighbors_on_axis(
                    &lv2,
                    world,
                    pos,
                    BlockDirection::from_cardinal_direction(lv),
                )
                .await;
            }
        }

        if bl3 != m {
            for l in 1..j {
                let lv7 = pos.offset_dir(lv.to_offset(), l);
                if let Some(mut lv8) = lvs[l as usize] {
                    lv8.attached = m;
                    world
                        .set_block_state(
                            &lv7,
                            lv8.to_state_id(&Block::TRIPWIRE),
                            BlockFlags::NOTIFY_ALL,
                        )
                        .await;
                    // if world.get_block(&lv7).await != Block::AIR {}
                }
            }
        }
    }

    #[allow(clippy::fn_params_excessive_bools)]
    async fn play_sound(
        world: &Arc<World>,
        block_pos: &BlockPos,
        attached: bool,
        on: bool,
        detached: bool,
        off: bool,
    ) {
        let cat = SoundCategory::Blocks;
        let pos = block_pos.to_f64();
        if on && !off {
            world
                .play_sound_raw(Sound::BlockTripwireClickOn as u16, cat, &pos, 0.4, 0.6)
                .await;
            // TODO world.emitGameEvent((Entity)null, GameEvent.BLOCK_ACTIVATE, pos);
        } else if !on && off {
            world
                .play_sound_raw(Sound::BlockTripwireClickOff as u16, cat, &pos, 0.4, 0.5)
                .await;
            // TODO world.emitGameEvent((Entity)null, GameEvent.BLOCK_DEACTIVATE, pos);
        } else if attached && !detached {
            world
                .play_sound_raw(Sound::BlockTripwireAttach as u16, cat, &pos, 0.4, 0.7)
                .await;
            // TODO world.emitGameEvent((Entity)null, GameEvent.BLOCK_ATTACH, pos);
        } else if !attached && detached {
            let pitch = 1.2 / (thread_rng().r#gen::<f32>() * 0.2 + 0.9);
            world
                .play_sound_raw(Sound::BlockTripwireDetach as u16, cat, &pos, 0.4, pitch)
                .await;
            // TODO world.emitGameEvent((Entity)null, GameEvent.BLOCK_DETACH, pos);
        }
    }

    pub async fn update_neighbors_on_axis(
        block: &Block,
        world: &Arc<World>,
        block_pos: BlockPos,
        direction: BlockDirection,
    ) {
        world.update_neighbor(&block_pos, block).await;
        world
            .update_neighbors(
                &block_pos.offset(direction.opposite().to_offset()),
                Some(direction),
            )
            .await;
    }
}
