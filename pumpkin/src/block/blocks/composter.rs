use std::sync::Arc;

use async_trait::async_trait;
use pumpkin_data::{
    Block, BlockState,
    block_properties::{BlockProperties, ComposterLikeProperties, EnumVariants, Integer0To8},
    entity::EntityType,
    item::Item,
};
use pumpkin_macros::pumpkin_block;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::{BlockStateId, chunk::TickPriority, item::ItemStack, world::BlockFlags};
use rand::Rng;
use uuid::Uuid;

use crate::{
    block::{pumpkin_block::PumpkinBlock, registry::BlockActionResult},
    entity::{Entity, item::ItemEntity, player::Player},
    server::Server,
    world::World,
};

#[pumpkin_block("minecraft:composter")]
pub struct ComposterBlock;

#[async_trait]
impl PumpkinBlock for ComposterBlock {
    async fn normal_use(
        &self,
        block: &Block,
        _player: &Player,
        location: BlockPos,
        _server: &Server,
        world: &Arc<World>,
    ) {
        let state_id = world.get_block_state_id(&location).await;
        let props = ComposterLikeProperties::from_state_id(state_id, block);
        if props.get_level() == 8 {
            self.clear_composter(world, location, state_id, block).await;
        }
    }

    async fn use_with_item(
        &self,
        block: &Block,
        _player: &Player,
        location: BlockPos,
        item: &Item,
        _server: &Server,
        world: &Arc<World>,
    ) -> BlockActionResult {
        let state_id = world.get_block_state_id(&location).await;
        let props = ComposterLikeProperties::from_state_id(state_id, block);
        let level = props.get_level();
        if level == 8 {
            self.clear_composter(world, location, state_id, block).await;
        }
        if level < 7 {
            if let Some(chance) = Self::item_to_increase_chance(item) {
                if level == 0 || rand::rng().random_bool(chance) {
                    self.update_level_composter(world, location, state_id, block, level + 1)
                        .await;
                }
            }
        }
        BlockActionResult::Consume
    }

    async fn on_scheduled_tick(&self, world: &Arc<World>, block: &Block, location: &BlockPos) {
        let state_id = world.get_block_state_id(location).await;
        let props = ComposterLikeProperties::from_state_id(state_id, block);
        let level = props.get_level();
        if level == 7 {
            self.update_level_composter(world, *location, state_id, block, level + 1)
                .await;
        }
    }

    async fn get_comparator_output(
        &self,
        block: &Block,
        _world: &World,
        _pos: &BlockPos,
        state: &BlockState,
    ) -> Option<u8> {
        let props = ComposterLikeProperties::from_state_id(state.id, block);
        Some(props.get_level())
    }
}

impl ComposterBlock {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn item_to_increase_chance(item: &Item) -> Option<f64> {
        match item {
            i if i == &Item::JUNGLE_LEAVES => Some(0.3),
            i if i == &Item::OAK_LEAVES => Some(0.3),
            i if i == &Item::SPRUCE_LEAVES => Some(0.3),
            i if i == &Item::DARK_OAK_LEAVES => Some(0.3),
            i if i == &Item::ACACIA_LEAVES => Some(0.3),
            i if i == &Item::CHERRY_LEAVES => Some(0.3),
            i if i == &Item::BIRCH_LEAVES => Some(0.3),
            i if i == &Item::AZALEA_LEAVES => Some(0.3),
            i if i == &Item::MANGROVE_LEAVES => Some(0.3),
            i if i == &Item::OAK_SAPLING => Some(0.3),
            i if i == &Item::SPRUCE_SAPLING => Some(0.3),
            i if i == &Item::BIRCH_SAPLING => Some(0.3),
            i if i == &Item::JUNGLE_SAPLING => Some(0.3),
            i if i == &Item::ACACIA_SAPLING => Some(0.3),
            i if i == &Item::CHERRY_SAPLING => Some(0.3),
            i if i == &Item::DARK_OAK_SAPLING => Some(0.3),
            i if i == &Item::MANGROVE_PROPAGULE => Some(0.3),
            i if i == &Item::BEETROOT_SEEDS => Some(0.3),
            i if i == &Item::DRIED_KELP => Some(0.3),
            i if i == &Item::SHORT_GRASS => Some(0.3),
            i if i == &Item::KELP => Some(0.3),
            i if i == &Item::MELON_SEEDS => Some(0.3),
            i if i == &Item::PUMPKIN_SEEDS => Some(0.3),
            i if i == &Item::SEAGRASS => Some(0.3),
            i if i == &Item::SWEET_BERRIES => Some(0.3),
            i if i == &Item::GLOW_BERRIES => Some(0.3),
            i if i == &Item::WHEAT_SEEDS => Some(0.3),
            i if i == &Item::MOSS_CARPET => Some(0.3),
            i if i == &Item::PINK_PETALS => Some(0.3),
            i if i == &Item::SMALL_DRIPLEAF => Some(0.3),
            i if i == &Item::HANGING_ROOTS => Some(0.3),
            i if i == &Item::MANGROVE_ROOTS => Some(0.3),
            i if i == &Item::TORCHFLOWER_SEEDS => Some(0.3),
            i if i == &Item::PITCHER_POD => Some(0.3),
            i if i == &Item::DRIED_KELP_BLOCK => Some(0.5),
            i if i == &Item::TALL_GRASS => Some(0.5),
            i if i == &Item::FLOWERING_AZALEA_LEAVES => Some(0.5),
            i if i == &Item::CACTUS => Some(0.5),
            i if i == &Item::SUGAR_CANE => Some(0.5),
            i if i == &Item::VINE => Some(0.5),
            i if i == &Item::NETHER_SPROUTS => Some(0.5),
            i if i == &Item::WEEPING_VINES => Some(0.5),
            i if i == &Item::TWISTING_VINES => Some(0.5),
            i if i == &Item::MELON_SLICE => Some(0.5),
            i if i == &Item::GLOW_LICHEN => Some(0.5),
            i if i == &Item::SEA_PICKLE => Some(0.65),
            i if i == &Item::LILY_PAD => Some(0.65),
            i if i == &Item::PUMPKIN => Some(0.65),
            i if i == &Item::CARVED_PUMPKIN => Some(0.65),
            i if i == &Item::MELON => Some(0.65),
            i if i == &Item::APPLE => Some(0.65),
            i if i == &Item::BEETROOT => Some(0.65),
            i if i == &Item::CARROT => Some(0.65),
            i if i == &Item::COCOA_BEANS => Some(0.65),
            i if i == &Item::POTATO => Some(0.65),
            i if i == &Item::WHEAT => Some(0.65),
            i if i == &Item::BROWN_MUSHROOM => Some(0.65),
            i if i == &Item::RED_MUSHROOM => Some(0.65),
            i if i == &Item::MUSHROOM_STEM => Some(0.65),
            i if i == &Item::CRIMSON_FUNGUS => Some(0.65),
            i if i == &Item::WARPED_FUNGUS => Some(0.65),
            i if i == &Item::NETHER_WART => Some(0.65),
            i if i == &Item::CRIMSON_ROOTS => Some(0.65),
            i if i == &Item::WARPED_ROOTS => Some(0.65),
            i if i == &Item::SHROOMLIGHT => Some(0.65),
            i if i == &Item::DANDELION => Some(0.65),
            i if i == &Item::POPPY => Some(0.65),
            i if i == &Item::BLUE_ORCHID => Some(0.65),
            i if i == &Item::ALLIUM => Some(0.65),
            i if i == &Item::AZURE_BLUET => Some(0.65),
            i if i == &Item::RED_TULIP => Some(0.65),
            i if i == &Item::ORANGE_TULIP => Some(0.65),
            i if i == &Item::WHITE_TULIP => Some(0.65),
            i if i == &Item::PINK_TULIP => Some(0.65),
            i if i == &Item::OXEYE_DAISY => Some(0.65),
            i if i == &Item::CORNFLOWER => Some(0.65),
            i if i == &Item::LILY_OF_THE_VALLEY => Some(0.65),
            i if i == &Item::WITHER_ROSE => Some(0.65),
            i if i == &Item::FERN => Some(0.65),
            i if i == &Item::SUNFLOWER => Some(0.65),
            i if i == &Item::LILAC => Some(0.65),
            i if i == &Item::ROSE_BUSH => Some(0.65),
            i if i == &Item::PEONY => Some(0.65),
            i if i == &Item::LARGE_FERN => Some(0.65),
            i if i == &Item::SPORE_BLOSSOM => Some(0.65),
            i if i == &Item::AZALEA => Some(0.65),
            i if i == &Item::MOSS_BLOCK => Some(0.65),
            i if i == &Item::BIG_DRIPLEAF => Some(0.65),
            i if i == &Item::HAY_BLOCK => Some(0.85),
            i if i == &Item::BROWN_MUSHROOM_BLOCK => Some(0.85),
            i if i == &Item::RED_MUSHROOM_BLOCK => Some(0.85),
            i if i == &Item::NETHER_WART_BLOCK => Some(0.85),
            i if i == &Item::WARPED_WART_BLOCK => Some(0.85),
            i if i == &Item::FLOWERING_AZALEA => Some(0.85),
            i if i == &Item::BREAD => Some(0.85),
            i if i == &Item::BAKED_POTATO => Some(0.85),
            i if i == &Item::COOKIE => Some(0.85),
            i if i == &Item::TORCHFLOWER => Some(0.85),
            i if i == &Item::PITCHER_PLANT => Some(0.85),
            i if i == &Item::CAKE => Some(1.0),
            i if i == &Item::PUMPKIN_PIE => Some(1.0),
            _ => None,
        }
    }

    pub async fn update_level_composter(
        &self,
        world: &Arc<World>,
        location: BlockPos,
        state_id: BlockStateId,
        block: &Block,
        level: u8,
    ) {
        let mut props = ComposterLikeProperties::from_state_id(state_id, block);
        props.set_level(level);
        world
            .set_block_state(&location, props.to_state_id(block), BlockFlags::NOTIFY_ALL)
            .await;
        if level == 7 {
            world
                .schedule_block_tick(block, location, 20, TickPriority::Normal)
                .await;
        }
    }

    pub async fn clear_composter(
        &self,
        world: &Arc<World>,
        location: BlockPos,
        state_id: BlockStateId,
        block: &Block,
    ) {
        self.update_level_composter(world, location, state_id, block, 0)
            .await;

        let item_position = {
            let mut rng = rand::rng();
            location.to_centered_f64().add_raw(
                rng.random_range(-0.35..=0.35),
                rng.random_range(-0.35..=0.35) + 0.51,
                rng.random_range(-0.35..=0.35),
            )
        };

        let item_entity = ItemEntity::new(
            Entity::new(
                Uuid::new_v4(),
                world.clone(),
                item_position,
                EntityType::ITEM,
                false,
            ),
            ItemStack::new(1, &Item::BONE_MEAL),
        )
        .await;

        world.spawn_entity(Arc::new(item_entity)).await;
    }
}

pub trait ComposterPropertiesEx {
    fn get_level(&self) -> u8;
    fn set_level(&mut self, level: u8);
}

impl ComposterPropertiesEx for ComposterLikeProperties {
    fn get_level(&self) -> u8 {
        self.level.to_index() as u8
    }
    fn set_level(&mut self, level: u8) {
        self.level = Integer0To8::from_index(u16::from(level));
    }
}
