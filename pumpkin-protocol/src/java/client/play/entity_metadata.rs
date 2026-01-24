use pumpkin_data::{meta_data_type::MetaDataType, packet::clientbound::PLAY_SET_ENTITY_DATA};
use pumpkin_macros::packet;
use serde::Serialize;

use crate::{VarInt, ser::network_serialize_no_prefix};

/// Updates the "Data Tracker" values for an entity.
///
/// Entity Metadata (or DataWatchers) controls persistent visual states that
/// don't require a full packet to update, such as whether an entity is on fire,
/// crouching, glowing, or the custom name displayed above its head.
#[derive(Serialize)]
#[packet(PLAY_SET_ENTITY_DATA)]
pub struct CSetEntityMetadata {
    /// The Entity ID of the entity whose metadata is being updated.
    pub entity_id: VarInt,
    /// A serialized collection of metadata entries.
    /// Ends with a terminal byte (0xFF).
    #[serde(serialize_with = "network_serialize_no_prefix")]
    pub metadata: Box<[u8]>,
}

impl CSetEntityMetadata {
    pub fn new(entity_id: VarInt, metadata: Box<[u8]>) -> Self {
        Self {
            entity_id,
            metadata,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Metadata<T> {
    index: u8,
    r#type: VarInt,
    value: T,
}

impl<T> Metadata<T> {
    pub fn new(index: u8, r#type: MetaDataType, value: T) -> Self {
        Self {
            index,
            r#type: VarInt(r#type as i32),
            value,
        }
    }
}
