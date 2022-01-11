use futures::{SinkExt, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

mod device;
mod socket;
pub mod types;

use crate::types::{ChannelName, FaderName};
pub use device::*;
pub use socket::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonRequest {
    Ping,
    Command(GoXLRCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonResponse {
    Ok(Option<DeviceStatus>),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoXLRCommand {
    GetStatus,
    AssignFader(FaderName, ChannelName),
}
