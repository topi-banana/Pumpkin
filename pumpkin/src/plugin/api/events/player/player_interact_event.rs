use std::sync::Arc;

use crate::entity::player::Player;
use pumpkin_macros::{Event, cancellable};
use pumpkin_protocol::server::play::ActionType;
use pumpkin_util::math::vector3::Vector3;

use super::PlayerEvent;

/// An event triggered when a player performs an interaction,
/// such as clicking, using an item, or interacting with a block.
///
/// Typically used to intercept and respond to player interaction logic,
/// such as right-clicking on a block or entity.
#[cancellable]
#[derive(Event, Clone)]
pub struct PlayerInteractEvent {
    /// The player who performed the interaction.
    pub player: Arc<Player>,

    /// The type of action the player performed.
    pub action: ActionType,

    /// The position in the world the player is looking at during the interaction.
    /// This is typically used to determine what block or location the player is targeting.
    pub looking_pos: Option<Vector3<f32>>,
}

impl PlayerInteractEvent {
    /// Creates a new instance of `PlayerInteractEvent`.
    ///
    /// # Arguments
    ///
    /// - `player`: The player who initiated the interaction.
    /// - `action`: The type of interaction performed.
    /// - `looking_pos`: The world position the player is looking at; `None` if not targeting anything.
    ///
    /// # Returns
    ///
    /// A new instance of `PlayerInteractEvent`.
    pub fn new(player: Arc<Player>, action: ActionType, looking_pos: Option<Vector3<f32>>) -> Self {
        Self {
            player,
            action,
            looking_pos,
            cancelled: false,
        }
    }
}

impl PlayerEvent for PlayerInteractEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}
