use serde::{Deserialize, Serialize};

#[must_use]
pub const fn to_long(float: f32) -> i64 {
    (float * 10000f32) as i64
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct NoiseValuePoint {
    pub temperature: i64,
    pub humidity: i64,
    pub continentalness: i64,
    pub erosion: i64,
    pub depth: i64,
    pub weirdness: i64,
}

impl NoiseValuePoint {
    #[must_use]
    pub const fn convert_to_list(&self) -> [i64; 7] {
        [
            self.temperature,
            self.humidity,
            self.continentalness,
            self.erosion,
            self.depth,
            self.weirdness,
            0,
        ]
    }
}

// Multi-noise sampler tests that need `VanillaGenerator`
// (`sample_value`, `sample_multinoise_biome`) live in
// `pumpkin-worldgen/tests/biome.rs`.
