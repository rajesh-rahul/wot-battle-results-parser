#![doc(html_root_url = "https://docs.rs/wot_replay_parser/0.2.2")]
//! A parser for `.wotreplay` files generated by the game World of Tanks

mod error;
mod utils;

mod replay_errors;
mod replay_parser;

mod packet_parser;
mod wot_data;
pub use packet_parser::{
    events, Context, EventPrinter, EventStream, EventType, Packet, PacketError, PacketParser, PacketStream,
};

mod battle_context;
pub use battle_context::BattleContext;
pub use error::ReplayError;
// TODO: Remove this * import
pub use replay_parser::*;

pub use crate::utils::get_replay_time;

pub mod wot_types {
    pub use wot_types::{ArenaBonusType, WotValue};
}

pub use packet_parser::PacketName;
pub use wot_data::VERSIONS;
