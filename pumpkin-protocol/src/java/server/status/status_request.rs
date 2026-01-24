use pumpkin_data::packet::serverbound::STATUS_STATUS_REQUEST;
use pumpkin_macros::packet;
use serde::Serialize;

/// Sent by the client to request the server's current status information.
///
/// This is the first packet sent during the "Status" state.
/// The server should respond with `CStatusResponse`.
#[derive(Serialize)]
#[packet(STATUS_STATUS_REQUEST)]
pub struct SStatusRequest;
