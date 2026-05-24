//! End-to-end smoke test that drives `generate_single_chunk` with a real
//! `VanillaGenerator`. Originally lived in
//! `pumpkin-world/src/chunk_system/generation.rs`; moved once `get_world_gen`
//! left `pumpkin-world`.

use std::sync::Arc;

use pumpkin_data::dimension::Dimension;
use pumpkin_util::world_seed::Seed;
use pumpkin_world::biome::hash_seed;
use pumpkin_world::chunk_system::{StagedChunkEnum, generate_single_chunk};
use pumpkin_world::world::WorldPortalExt;
use pumpkin_worldgen::get_world_gen;

struct BlockRegistry;

impl WorldPortalExt for BlockRegistry {
    fn can_place_at(
        &self,
        _block: &pumpkin_data::Block,
        _state: &pumpkin_data::BlockState,
        _block_accessor: &dyn pumpkin_world::world::BlockAccessor,
        _block_pos: &pumpkin_util::math::position::BlockPos,
    ) -> bool {
        true
    }

    fn spawn_mobs_for_chunk_generation(
        &self,
        _cache: &mut dyn pumpkin_world::chunk_system::generation_cache::GenerationCache,
        _biome: &'static pumpkin_data::chunk::Biome,
        _chunk_x: i32,
        _chunk_z: i32,
    ) {
    }
}

#[test]
fn generate_chunk_should_return() {
    let dimension = Dimension::OVERWORLD;
    let seed = Seed(42);
    let block_registry = Arc::new(BlockRegistry);
    let world_gen = get_world_gen(seed, dimension.clone());
    let biome_mixer_seed = hash_seed(seed.0);

    let _ = generate_single_chunk(
        &dimension,
        biome_mixer_seed,
        &*world_gen,
        block_registry.as_ref(),
        0,
        0,
        StagedChunkEnum::Full,
    );
}
