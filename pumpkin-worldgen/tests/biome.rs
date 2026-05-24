//! Integration tests for biome supplier + `VanillaGenerator`.
//! Originally lived in `pumpkin-world/src/biome/{mod,multi_noise}.rs`;
//! moved here once `VanillaGenerator` left `pumpkin-world`.

use std::path::PathBuf;

use pumpkin_data::{chunk::Biome, dimension::Dimension};
use pumpkin_util::serde_json;
use pumpkin_util::world_seed::Seed;
use pumpkin_world::ProtoChunk;
use pumpkin_world::biome::{BiomeSupplier, MultiNoiseBiomeSupplier};
use pumpkin_world::generation::biome_coords;
use pumpkin_world::generation::noise::router::multi_noise_sampler::{
    MultiNoiseSampler, MultiNoiseSamplerBuilderOptions,
};
use pumpkin_world::generation::positions::chunk_pos;
use pumpkin_worldgen::vanilla::{GeneratorInit, VanillaGenerator};
use serde::Deserialize;

fn asset(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../pumpkin-world/assets/{name}"))
}

fn read_json<T: serde::de::DeserializeOwned>(name: &str) -> T {
    let raw = std::fs::read_to_string(asset(name)).expect("asset file missing");
    serde_json::from_str(&raw).expect("failed to decode asset JSON")
}

#[test]
fn biome_desert() {
    let seed = 13579;
    let generator = VanillaGenerator::new(Seed(seed as u64), Dimension::OVERWORLD);
    let multi_noise_config = MultiNoiseSamplerBuilderOptions::new(1, 1, 1);
    let mut sampler =
        MultiNoiseSampler::generate(&generator.base_router.multi_noise, &multi_noise_config);
    let biome = MultiNoiseBiomeSupplier::OVERWORLD.biome(-24, 1, 8, &mut sampler);
    assert_eq!(biome, &Biome::DESERT);
}

#[test]
fn wide_area_surface() {
    #[derive(Deserialize)]
    struct BiomeData {
        x: i32,
        z: i32,
        data: Vec<(i32, i32, i32, u8)>,
    }

    let expected_data: Vec<BiomeData> = read_json("biome_no_blend_no_beard_0.json");

    let seed = 0;
    let generator = VanillaGenerator::new(Seed(seed as u64), Dimension::OVERWORLD);

    for data in expected_data {
        let chunk_x = data.x;
        let chunk_z = data.z;

        let mut chunk = ProtoChunk::new(chunk_x, chunk_z, &generator);

        let start_x = chunk_pos::start_block_x(chunk_x);
        let start_z = chunk_pos::start_block_z(chunk_z);

        let horizontal_biome_end = biome_coords::from_block(16);
        let multi_noise_config = MultiNoiseSamplerBuilderOptions::new(
            biome_coords::from_block(start_x),
            biome_coords::from_block(start_z),
            horizontal_biome_end as usize,
        );
        let mut multi_noise_sampler =
            MultiNoiseSampler::generate(&generator.base_router.multi_noise, &multi_noise_config);

        chunk.populate_biomes(&generator, &mut multi_noise_sampler);

        for (biome_x, biome_y, biome_z, biome_id) in data.data {
            let calculated_biome = chunk.get_biome(biome_x, biome_y, biome_z);

            assert_eq!(
                biome_id, calculated_biome.id,
                "Expected {:?} was {:?} at {},{},{} ({},{})",
                Biome::from_id(biome_id),
                calculated_biome,
                biome_x,
                biome_y,
                biome_z,
                data.x,
                data.z
            );
        }
    }
}

#[test]
fn sample_value() {
    type PosToPoint = (i32, i32, i32, i64, i64, i64, i64, i64, i64);
    let expected_data: Vec<PosToPoint> =
        read_json("multi_noise_sample_no_blend_no_beard_0_0_0.json");

    let seed = 0;
    let chunk_x = 0;
    let chunk_z = 0;

    let generator = VanillaGenerator::new(Seed(seed as u64), Dimension::OVERWORLD);

    let _chunk = ProtoChunk::new(chunk_x, chunk_z, &generator);

    let start_x = chunk_pos::start_block_x(chunk_x);
    let start_z = chunk_pos::start_block_z(chunk_z);
    let horizontal_biome_end = biome_coords::from_block(16);
    let multi_noise_config = MultiNoiseSamplerBuilderOptions::new(
        biome_coords::from_block(start_x),
        biome_coords::from_block(start_z),
        horizontal_biome_end as usize,
    );
    let mut multi_noise_sampler =
        MultiNoiseSampler::generate(&generator.base_router.multi_noise, &multi_noise_config);

    for (x, y, z, tem, hum, con, ero, dep, wei) in expected_data {
        let point = multi_noise_sampler.sample(x, y, z);
        assert_eq!(point.temperature, tem);
        assert_eq!(point.humidity, hum);
        assert_eq!(point.continentalness, con);
        assert_eq!(point.erosion, ero);
        assert_eq!(point.depth, dep);
        assert_eq!(point.weirdness, wei);
    }
}

#[test]
fn sample_multinoise_biome() {
    let expected_data: Vec<(i32, i32, i32, u8)> = read_json("multi_noise_biome_source_test.json");

    let seed = 0;
    let generator = VanillaGenerator::new(Seed(seed as u64), Dimension::OVERWORLD);

    let mut sampler = MultiNoiseSampler::generate(
        &generator.base_router.multi_noise,
        &MultiNoiseSamplerBuilderOptions::new(0, 0, 4),
    );

    for (x, y, z, biome_id) in expected_data {
        let calculated_biome = MultiNoiseBiomeSupplier::OVERWORLD.biome(x, y, z, &mut sampler);

        assert_eq!(
            biome_id, calculated_biome.id,
            "Expected {:?} was {:?} at {},{},{}",
            Biome::from_id(biome_id),
            calculated_biome,
            x,
            y,
            z
        );
    }
}
