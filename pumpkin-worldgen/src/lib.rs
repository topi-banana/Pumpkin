pub mod structure_finder;
pub mod vanilla;

use std::sync::Arc;

use pumpkin_data::{
    Block, BlockState, chunk_gen_settings::GenerationSettings, dimension::Dimension,
};
use pumpkin_util::math::vector2::Vector2;
use pumpkin_util::world_seed::Seed;
use pumpkin_world::generation::biome_coords;
use pumpkin_world::generation::noise::aquifer_sampler::FluidLevel;
use pumpkin_world::generation::noise::router::multi_noise_sampler::{
    MultiNoiseSampler, MultiNoiseSamplerBuilderOptions,
};
use pumpkin_world::generation::noise::router::proto_noise_router::ProtoNoiseRouters;
use pumpkin_world::generation::noise::router::surface_height_sampler::{
    SurfaceHeightEstimateSampler, SurfaceHeightSamplerBuilderOptions,
};
use pumpkin_world::generation::noise::{CHUNK_DIM, ChunkNoiseGenerator};
use pumpkin_world::generation::positions::chunk_pos;
use pumpkin_world::generation::proto_chunk::{StandardChunkFluidLevelSampler, TerrainCache};
use pumpkin_world::generation::GlobalRandomConfig;
use pumpkin_world::generator::Generator;
use pumpkin_world::ProtoChunk;

use crate::vanilla::{GeneratorInit, VanillaGenerator};

/// Construct the default world generator for the given dimension.
#[must_use]
pub fn get_world_gen(seed: Seed, dimension: Dimension) -> Box<dyn Generator> {
    // TODO decide which WorldGenerator to pick based on config.
    Box::new(VanillaGenerator::new(seed, dimension))
}

/// Convenience: returns the default world generator as an `Arc<dyn Generator>`.
#[must_use]
pub fn vanilla_arc(seed: Seed, dimension: Dimension) -> Arc<dyn Generator> {
    Arc::from(get_world_gen(seed, dimension))
}

pub fn bench_create_and_populate_noise(
    _base_router: &ProtoNoiseRouters,
    random_config: &GlobalRandomConfig,
    _settings: &GenerationSettings,
    _terrain_cache: &TerrainCache,
    _default_block: &'static BlockState,
) {
    let generator = VanillaGenerator::new(Seed(random_config.seed), Dimension::OVERWORLD);
    let mut chunk = ProtoChunk::new(0, 0, &generator);

    let settings = generator.settings;
    let generation_shape = &settings.shape;
    let horizontal_cell_count = CHUNK_DIM / generation_shape.horizontal_cell_block_count();
    let sampler = StandardChunkFluidLevelSampler::new(
        FluidLevel::new(
            settings.sea_level,
            Block::from_state_id(settings.default_fluid.id),
        ),
        FluidLevel::new(-54, &Block::LAVA),
    );

    let start_x = chunk_pos::start_block_x(0);
    let start_z = chunk_pos::start_block_z(0);

    let mut noise_sampler = ChunkNoiseGenerator::new(
        &generator.base_router.noise,
        &generator.random_config,
        horizontal_cell_count as usize,
        start_x,
        start_z,
        generation_shape,
        sampler,
        settings.aquifers_enabled,
        settings.ore_veins_enabled,
    );

    let biome_pos = Vector2::new(
        biome_coords::from_block(start_x),
        biome_coords::from_block(start_z),
    );
    let horizontal_biome_end = biome_coords::from_block(
        horizontal_cell_count as i32 * generation_shape.horizontal_cell_block_count() as i32,
    );
    let surface_config = SurfaceHeightSamplerBuilderOptions::new(
        biome_pos.x,
        biome_pos.y,
        horizontal_biome_end as usize,
        generation_shape.min_y as i32,
        generation_shape.max_y() as i32,
        generation_shape.vertical_cell_block_count() as usize,
    );
    let mut surface_height_estimate_sampler = SurfaceHeightEstimateSampler::generate(
        &generator.base_router.surface_estimator,
        &surface_config,
    );

    chunk.populate_noise(
        &generator,
        &mut noise_sampler,
        &generator.random_config.ore_random_deriver,
        &mut surface_height_estimate_sampler,
    );
}

pub fn bench_create_and_populate_biome(
    _base_router: &ProtoNoiseRouters,
    random_config: &GlobalRandomConfig,
    _settings: &GenerationSettings,
    _terrain_cache: &TerrainCache,
    _default_block: &'static BlockState,
) {
    let generator = VanillaGenerator::new(Seed(random_config.seed), Dimension::OVERWORLD);
    let mut chunk = ProtoChunk::new(0, 0, &generator);

    let start_x = chunk_pos::start_block_x(0);
    let start_z = chunk_pos::start_block_z(0);
    let biome_pos = Vector2::new(
        biome_coords::from_block(start_x),
        biome_coords::from_block(start_z),
    );
    let horizontal_biome_end = biome_coords::from_block(16);
    let multi_noise_config = MultiNoiseSamplerBuilderOptions::new(
        biome_pos.x,
        biome_pos.y,
        horizontal_biome_end as usize,
    );
    let mut multi_noise_sampler =
        MultiNoiseSampler::generate(&generator.base_router.multi_noise, &multi_noise_config);

    chunk.populate_biomes(&generator, &mut multi_noise_sampler);
}

pub fn bench_create_and_populate_noise_with_surface(
    _base_router: &ProtoNoiseRouters,
    random_config: &GlobalRandomConfig,
    _settings: &GenerationSettings,
    _terrain_cache: &TerrainCache,
    _default_block: &'static BlockState,
) {
    let generator = VanillaGenerator::new(Seed(random_config.seed), Dimension::OVERWORLD);
    let mut chunk = ProtoChunk::new(0, 0, &generator);

    let settings = generator.settings;
    let generation_shape = &settings.shape;
    let horizontal_cell_count = CHUNK_DIM / generation_shape.horizontal_cell_block_count();
    let start_x = chunk_pos::start_block_x(0);
    let start_z = chunk_pos::start_block_z(0);

    let biome_pos = Vector2::new(
        biome_coords::from_block(start_x),
        biome_coords::from_block(start_z),
    );
    let horizontal_biome_end = biome_coords::from_block(16);
    let multi_noise_config = MultiNoiseSamplerBuilderOptions::new(
        biome_pos.x,
        biome_pos.y,
        horizontal_biome_end as usize,
    );
    let mut multi_noise_sampler =
        MultiNoiseSampler::generate(&generator.base_router.multi_noise, &multi_noise_config);

    let sampler = StandardChunkFluidLevelSampler::new(
        FluidLevel::new(
            settings.sea_level,
            Block::from_state_id(settings.default_fluid.id),
        ),
        FluidLevel::new(-54, &Block::LAVA),
    );

    let mut noise_sampler = ChunkNoiseGenerator::new(
        &generator.base_router.noise,
        &generator.random_config,
        horizontal_cell_count as usize,
        start_x,
        start_z,
        generation_shape,
        sampler,
        settings.aquifers_enabled,
        settings.ore_veins_enabled,
    );

    let surface_config = SurfaceHeightSamplerBuilderOptions::new(
        biome_pos.x,
        biome_pos.y,
        horizontal_biome_end as usize,
        generation_shape.min_y as i32,
        generation_shape.max_y() as i32,
        generation_shape.vertical_cell_block_count() as usize,
    );
    let mut surface_height_estimate_sampler = SurfaceHeightEstimateSampler::generate(
        &generator.base_router.surface_estimator,
        &surface_config,
    );

    chunk.populate_biomes(&generator, &mut multi_noise_sampler);
    chunk.populate_noise(
        &generator,
        &mut noise_sampler,
        &generator.random_config.ore_random_deriver,
        &mut surface_height_estimate_sampler,
    );
    chunk.build_surface(&generator, &mut surface_height_estimate_sampler);
}
