use pumpkin_data::BlockState;
use pumpkin_data::dimension::Dimension;
use pumpkin_data::structures::StructurePlacement;
use pumpkin_util::math::position::BlockPos;

use crate::ProtoChunk;
use crate::chunk_system::generation_cache::Cache;
use crate::world::WorldPortalExt;

/// Abstraction over a world generator. Implementations encapsulate the noise
/// router, structure placement, biome supplier, and stage-driver logic for a
/// particular generator flavour (vanilla, flat, custom, ...). [`Level`] holds
/// the chosen generator as `Arc<dyn Generator>`.
pub trait Generator: Send + Sync {
    /// The dimension this generator targets.
    fn dimension(&self) -> &Dimension;

    /// The world seed.
    fn seed(&self) -> u64;

    /// Default block used to fill freshly-allocated proto-chunks.
    fn default_block(&self) -> &'static BlockState;

    /// Seed used to mix biomes across chunks.
    fn biome_mixer_seed(&self) -> i64;

    // --- Stage drivers (called by `Cache::advance`) ---

    fn step_biomes(&self, chunk: &mut ProtoChunk);
    fn step_noise(&self, chunk: &mut ProtoChunk);
    fn step_surface(&self, chunk: &mut ProtoChunk);
    fn step_carvers(&self, chunk: &mut ProtoChunk);
    fn set_structure_starts(&self, chunk: &mut ProtoChunk);
    fn set_structure_references(&self, chunk: &mut ProtoChunk);

    fn generate_features_and_structure(
        &self,
        cache: &mut Cache,
        block_registry: &dyn WorldPortalExt,
    );
    fn spawn_mobs(&self, cache: &mut Cache, block_registry: &dyn WorldPortalExt);

    /// Vanilla `ChunkGenerator.findNearestMapStructure` analogue.
    fn find_nearest_structure(
        &self,
        origin: BlockPos,
        placements: &[&StructurePlacement],
        max_search_radius: i32,
    ) -> Option<BlockPos>;
}
