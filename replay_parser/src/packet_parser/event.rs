use std::collections::HashMap;

use entity_method::parse_entity_method;
use serde::Serialize;

use crate::events::*;
use crate::packet_parser::{Context, Packet, PacketError};
use crate::ReplayError;

/// This enum aims to represent all possible events that can occur in a battle. Variants of this enum
/// usually maps to a specific type of packet. For example, `BattleEvent::Position` maps to packets of
/// type `0x0A`. There are special cases however, such as `BattleEvent::GameVersion` which maps to packets of
/// different type depending on the game version:
/// - `0x14` (if the replay is from, or before patch 0.9.13)
/// - `0x18` (if the replay is from or later patch 0.9.14)
///
/// Some of these events have sub-events. For example, `BattleEvent::EntityMethod` has many sub events
/// (represented as enum variants) such as `Avatar`, `Vehicle` etc. Furthermore, these sub-events also have
/// their own sub-events. See their documentation for details
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
// TODO: Box Large structure
#[serde(tag = "name", content = "data")]
pub enum BattleEvent {
    Unimplemented { packet_type: u32, size: usize },
    GameVersion(GameVersion),
    CreateAvatar(CreateAvatar),
    // EntityMethod(EntityMethodEvent),
    Position(Position),
    Chat(Chat),
    CreateEntity(CreateEntity),
    // EntityProperty(EntityPropertyEvent),
    CryptoKey(CryptoKey),
    AvatarReady(AvatarReady),
    UnimplementedEntityMethod(EntMethodGeneralInfo),
    VehicleAdded(VehicleAdded),
    UnimplementedArenaUpdate(String),
    BasePoints(BasePoints),
    VehicleList(VehicleList),
    BaseCaptured(BaseCaptured),
    FogOfWar(FogOfWar),
    VehicleDescr(VehicleDescr),
    VehicleStatistics(VehicleStatistics),
    Statistics(Statistics),
    VehicleKilled(VehicleKilled),
    VehicleUpdated(VehicleUpdated),
    Period(Period),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, strum::Display)]
pub enum PacketName {
    GameVersion,
    CreateAvatar,
    CreateEntity,
    Position,
    Chat,
    CryptoKey,
    Unimplemented,
    EntityMethod,
}

impl BattleEvent {
    /// Parse packet to a Battle event. Optional context is provided to aid in parsing some particular
    /// packets.
    pub fn parse(packet: &Packet, context: &mut Context) -> Result<BattleEvent, ReplayError> {
        let packet_type = packet.packet_type();

        let packet_name = context.version_data.packet_name(packet.packet_type());

        let event_result = match packet_name {
            PacketName::GameVersion => GameVersion::parse(packet, context),
            PacketName::CreateAvatar => CreateAvatar::parse_mut(packet, context),
            PacketName::CreateEntity => CreateEntity::parse_mut(packet, context),
            PacketName::Position => Position::parse(packet, context),
            PacketName::Chat => Chat::parse(packet, context),
            PacketName::CryptoKey => CryptoKey::parse(packet, context),
            PacketName::EntityMethod => parse_entity_method(packet, context),
            PacketName::Unimplemented => {
                return Ok(BattleEvent::Unimplemented {
                    packet_type,
                    size: packet.payload().len(),
                })
            }
        };

        event_result.map_err(|error| ReplayError::PacketParseError {
            packet_id: packet.id(),
            packet_type: packet.packet_type(),
            packet_name,
            error,
        })
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, BattleEvent::Unimplemented { .. })
    }

    pub fn corresponding_pkt_name(&self) -> PacketName {
        use BattleEvent::*;
        match self {
            Unimplemented { .. } => PacketName::Unimplemented,
            GameVersion(_) => PacketName::GameVersion,
            CreateAvatar(_) => PacketName::CreateAvatar,
            Position(_) => PacketName::Position,
            Chat(_) => PacketName::Chat,
            CreateEntity(_) => PacketName::CreateEntity,
            CryptoKey(_) => PacketName::CryptoKey,
            UnimplementedEntityMethod(_)
            | AvatarReady(_)
            | VehicleDescr(_)
            | VehicleAdded(_)
            | UnimplementedArenaUpdate(_)
            | BasePoints(_)
            | VehicleList(_)
            | BaseCaptured(_)
            | FogOfWar(_)
            | VehicleStatistics(_)
            | Statistics(_)
            | Period(_)
            | VehicleKilled(_)
            | VehicleUpdated(_) => PacketName::EntityMethod,
        }
    }

    // pub fn entity_property(self) -> Option<EntityProperty> {
    //     match self {
    //         Self::EntityProperty(EntityPropertyEvent { property, .. }) => Some(property),
    //         _ => None,
    //     }
    // }
}

/// This trait is implemented by all events so that they can parse a packet to a BattleEvent
pub trait PacketParser {
    fn parse(_: &Packet, _: &Context) -> Result<BattleEvent, PacketError> {
        unimplemented!()
    }

    fn parse_mut(_: &Packet, _: &mut Context) -> Result<BattleEvent, PacketError> {
        unimplemented!()
    }
}

/// Used for debugging purposes. Instead of the `Debug` trait (we don't have to choose. It is available as
/// well) because its useful for us to transform some values based on the `BattleContext`. For example, an
/// event may have an `attacker_id` attribute. We can transform that id to the actual player.
/// Right now, the following options are used:
///  - `#[event_debug(ignore)]` - ignore that field when printing the event out
///  - `#[event_debug(as_player)]` - transform this field's value to player if possible
///  - `#[event_debug(call_debug_string)]` - This field has its own implementation of `EventPrinter` so call
///    that
///
/// If no options, then `std::fmt::Debug` is called on that field
pub trait EventPrinter {
    fn to_debug_string(&self, context: &Context) -> String
    where
        Self: std::fmt::Debug;
}

pub trait TrackVersion {
    fn name() -> &'static str;
    fn version() -> VersionInfo;
}

// impl EventPrinter for BattleEvent {
//     fn to_debug_string(&self, context: &Context) -> String
//     where
//         Self: std::fmt::Debug,
//     {
//         use BattleEvent::*;
//         match self {
//             Unimplemented { packet_type, size } => {
//                 format!("Unimplemented packet: {packet_type}, Size: {size}")
//             }
//             CreateAvatar(x) => x.to_debug_string(context),
//             GameVersion(x) => x.to_debug_string(context),
//             EntMethodGeneralInfo(x) => x.to_debug_string(context),
//             // EntityProperty(x) => x.to_debug_string(context),
//             Position(x) => x.to_debug_string(context),
//             Chat(x) => x.to_debug_string(context),
//             CreateEntity(x) => x.to_debug_string(context),
//             CryptoKey(x) => x.to_debug_string(context),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum VersionList {
    Range(([u16; 4], [u16; 4])),
    From([u16; 4]),
}

#[derive(Debug, Clone)]
pub enum VersionInfo {
    /// Present in all versions
    All,

    /// Present in this version
    Version([u16; 4]),

    /// Present in this version range
    VersionRange(([u16; 4], [u16; 4])),

    /// Intermittently present in these ranges
    VersionRangeList(&'static [VersionList]),

    /// Represent Versions of structs
    Struct(&'static [VersionInfo]),
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// All code related to the event stream
///////////////////////////////////////////////////////////////////////////////////////////////////
use super::PacketStream;
use crate::utils::validate_version;

pub struct EventStream<'pkt> {
    packet_stream: PacketStream<'pkt>,
    context:       Context,
}

impl<'pkt> EventStream<'pkt> {
    pub fn new(packet_stream: PacketStream<'pkt>, version: [u16; 4]) -> Self {
        let version_validated = validate_version(version);
        let context = Context::new(version_validated, HashMap::new());

        EventStream {
            packet_stream,
            context,
        }
    }
}

impl<'pkt> Iterator for EventStream<'pkt> {
    type Item = Result<BattleEvent, ReplayError>;

    fn next(&mut self) -> Option<Self::Item> {
        let packet = self.packet_stream.next()?;
        match packet {
            Ok(packet) => {
                let packet_id = packet.id();
                let event = BattleEvent::parse(&packet, &mut self.context);

                log_if_error(packet_id, &event);
                Some(event)
            }
            Err(err) => Some(Err(err)),
        }
    }
}

fn log_if_error(packet_id: i32, event: &Result<BattleEvent, ReplayError>) {
    match event.as_ref() {
        Ok(_) => {}
        Err(err) => {
            tracing::error!(packet_id, error = ?err)
        }
    }
}
