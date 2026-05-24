use pumpkin_data::BlockState;
use pumpkin_data::chunk_gen_settings::GenerationSettings;
use pumpkin_data::dimension::Dimension;
use pumpkin_data::noise_router::{
    END_BASE_NOISE_ROUTER, NETHER_BASE_NOISE_ROUTER, OVERWORLD_BASE_NOISE_ROUTER,
};
use pumpkin_data::structures::{StructurePlacement, StructurePlacementCalculator, StructureSet};
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::world_seed::Seed;
use pumpkin_world::ProtoChunk;
use pumpkin_world::chunk_system::generation_cache::Cache;
use pumpkin_world::generation::GlobalRandomConfig;
use pumpkin_world::generation::noise::router::proto_noise_router::ProtoNoiseRouters;
use pumpkin_world::generation::proto_chunk::TerrainCache;
use pumpkin_world::generation::structure::placement::GlobalStructureCache;
use pumpkin_world::generator::Generator;
use pumpkin_world::world::WorldPortalExt;
use rustc_hash::FxHashMap;

use crate::structure_finder::find_nearest_structure;

pub trait GeneratorInit {
    fn new(seed: Seed, dimension: Dimension) -> Self;
}

pub struct VanillaGenerator {
    pub random_config: GlobalRandomConfig,
    pub base_router: ProtoNoiseRouters,
    pub dimension: Dimension,
    pub settings: &'static GenerationSettings,
    pub biome_mixer_seed: i64,

    pub terrain_cache: TerrainCache,

    pub default_block: &'static BlockState,

    pub global_structure_cache: GlobalStructureCache,
    pub structure_calculator: StructurePlacementCalculator,
    pub structure_allowed_biomes: FxHashMap<usize, Vec<u16>>,
}

impl Generator for VanillaGenerator {
    fn dimension(&self) -> &Dimension {
        &self.dimension
    }

    fn seed(&self) -> u64 {
        self.random_config.seed
    }

    fn default_block(&self) -> &'static BlockState {
        self.default_block
    }

    fn biome_mixer_seed(&self) -> i64 {
        self.biome_mixer_seed
    }

    fn settings(&self) -> &'static GenerationSettings {
        self.settings
    }

    fn random_config(&self) -> &GlobalRandomConfig {
        &self.random_config
    }

    fn terrain_cache(&self) -> &TerrainCache {
        &self.terrain_cache
    }

    fn base_router(&self) -> &ProtoNoiseRouters {
        &self.base_router
    }

    fn structure_calculator(&self) -> &StructurePlacementCalculator {
        &self.structure_calculator
    }

    fn structure_allowed_biomes(&self) -> &FxHashMap<usize, Vec<u16>> {
        &self.structure_allowed_biomes
    }

    fn global_structure_cache(&self) -> &GlobalStructureCache {
        &self.global_structure_cache
    }

    fn step_biomes(&self, chunk: &mut ProtoChunk) {
        chunk.step_to_biomes(self);
    }

    fn step_noise(&self, chunk: &mut ProtoChunk) {
        chunk.step_to_noise(self);
    }

    fn step_surface(&self, chunk: &mut ProtoChunk) {
        chunk.step_to_surface(self);
    }

    fn step_carvers(&self, chunk: &mut ProtoChunk) {
        chunk.step_to_carvers(self);
    }

    fn set_structure_starts(&self, chunk: &mut ProtoChunk) {
        chunk.set_structure_starts(self);
    }

    fn set_structure_references(&self, chunk: &mut ProtoChunk) {
        chunk.set_structure_references(self);
    }

    fn generate_features_and_structure(
        &self,
        cache: &mut Cache,
        block_registry: &dyn WorldPortalExt,
    ) {
        ProtoChunk::generate_features_and_structure(cache, block_registry, &self.random_config);
    }

    fn spawn_mobs(&self, cache: &mut Cache, block_registry: &dyn WorldPortalExt) {
        ProtoChunk::spawn_mobs(cache, block_registry);
    }

    fn find_nearest_structure(
        &self,
        origin: BlockPos,
        placements: &[&StructurePlacement],
        max_search_radius: i32,
    ) -> Option<BlockPos> {
        find_nearest_structure(
            origin,
            placements,
            max_search_radius,
            self.random_config.seed as i64,
            &self.global_structure_cache,
        )
    }
}

impl GeneratorInit for VanillaGenerator {
    fn new(seed: Seed, dimension: Dimension) -> Self {
        let settings = GenerationSettings::from_dimension(&dimension);
        let random_config = GlobalRandomConfig::new(seed.0, settings.legacy_random_source);

        // TODO: The generation settings contains (part of?) the noise routers too; do we keep the separate or
        // use only the generation settings?
        let base = if dimension == Dimension::OVERWORLD {
            OVERWORLD_BASE_NOISE_ROUTER
        } else if dimension == Dimension::THE_NETHER {
            NETHER_BASE_NOISE_ROUTER
        } else if dimension == Dimension::THE_END {
            END_BASE_NOISE_ROUTER
        } else {
            tracing::error!("Unsupported dimension for noise router: {:?}", dimension);
            OVERWORLD_BASE_NOISE_ROUTER
        };
        let terrain_cache = TerrainCache::from_random(&random_config);

        let default_block = settings.default_block;
        let base_router = ProtoNoiseRouters::generate(&base, &random_config);
        let biome_mixer_seed = pumpkin_world::biome::hash_seed(seed.0);

        let mut structure_allowed_biomes = FxHashMap::default();
        for (i, set) in StructureSet::ALL.iter().enumerate() {
            structure_allowed_biomes.insert(i, ProtoChunk::get_allowed_biomes(set));
        }

        Self {
            random_config,
            base_router,
            dimension,
            settings,
            biome_mixer_seed,
            terrain_cache,
            default_block,
            global_structure_cache: GlobalStructureCache::new(),
            structure_calculator: StructurePlacementCalculator::new(seed.0 as i64),
            structure_allowed_biomes,
        }
    }
}
