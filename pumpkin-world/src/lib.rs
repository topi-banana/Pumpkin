pub mod biome;
pub mod blending_data;
pub mod block;
pub mod carving_mask;
pub mod chunk;
pub mod chunk_system;
pub mod cylindrical_chunk_iterator;
pub mod data;
pub mod dimension;
pub mod generation;
pub mod generator;
pub mod height_limit;
pub mod inventory;
pub mod level;
pub mod lighting;
pub mod poi;
pub mod tick;
pub mod world;
pub mod world_info;

pub type BlockId = u16;
pub type BlockStateId = u16;

pub const CURRENT_MC_VERSION: &str = "26.1";

pub const CURRENT_BEDROCK_MC_VERSION: &str = "1.26.20";
pub const CURRENT_BEDROCK_MC_PROTOCOL: u32 = 975;

#[macro_export]
macro_rules! global_path {
    ($path:expr) => {{
        use std::path::Path;
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(file!())
            .parent()
            .unwrap()
            .join($path)
    }};
}

pub use generation::{
    GlobalRandomConfig, noise::router::proto_noise_router::ProtoNoiseRouters,
    proto_chunk::ProtoChunk,
};
