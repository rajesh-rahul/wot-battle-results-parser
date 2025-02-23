/// An incomplete implementation of serde Deserializer for some packets found in World of Tanks Replays.
///
/// All it achieves is instead of writing code like this:
/// ```ignore
/// let x = input.read_f32::<LE>().unwrap();
/// let z = input.read_f32::<LE>().unwrap();
/// let y = input.read_f32::<LE>().unwrap();
/// ```
///
/// We can write this:
/// ```ignore
/// struct Position {
///     x: f32, z: f32, y: f32
/// }
///
/// let position_packet: Position = serde_packet::from_slice(input).unwrap();
/// ```
/// This way `serde` takes care of the tedious byte to byte parsing (powered by `nom`).
///
/// Deserializer assumes that the input will match the data format. This means that a packet's payload must be
/// truncated to match the data.
///
/// A replay packet contains the following :
///     - Metadata - (packet type, payload size, timestamp)
///     - Payload
///
/// We can further divide the Payload depending on the packet type.
///
///
/// For example if we have the following packet (type is `0x08`) payload:
/// `008F2F48 00000000 00000002 0001` the input is only actually `0001` because it has the following format:
///     - `008F2F48` Object/Entity ID (Always part of the 0x08)
///     - `00000000` For packet `0x08`, this is known as the `messageID` inside BigWorld (the game engine WoT
///       for stuff that does not include graphics). For us, it tell us which method of the entity is being
///       called.
///     In this particular case, the `messageID` is 0 and refers to `showShooting()` method on the entity (the
/// entity is     `Vehicle` for this ex.)
///     - `00000002` tells us how many bytes of information is left. In this case this is `2` bytes
///     - `0001` is the interesting part: the input for serde. In this particular case it is simply two values
///       of u8s
mod serde_packet;


/// Contains code for all the different types of events. For each packet, we have an event. An event can be
/// considered the human readable abstraction over a packet.
pub mod events;

/// Contains `Packet`, and `PacketStream`. A light zero-copy wrapper for the binary data from
/// `.wotreplay`. These are then used by the `events` module to parse into events that we can understand.
mod packet;
pub use packet::{Packet, PacketStream};
mod input_reader;
pub(crate) use input_reader::InputStream;

mod error;
pub use error::PacketError;

mod context;
pub use context::Context;

mod update_arena;

mod event;
pub use event::{BattleEvent, EventPrinter, EventStream, EventType, PacketName, PacketParser};

pub mod types;

/// This is a trait used to make parsing EntityMethods and EntityProperties easier. Without this we
/// will need huge match statements everywhere and lot of repetitive code. This trait should be implemented by
/// deriving the `EnumVariantDeserialize` macro. Then, we use the following attributes on the enum variants to
/// parse that event from the packet:
///
/// - `#[variant_de(delegate)]` This simply calls `serde_packet::from_slice` on the type of the enum variant
/// - `#[variant_de(manual)]` This will just don't do anything (panic). Useful for when we need to provide a
///   manual way of parsing that cannot be handled by this trait (and the macro)
/// - no attribute and tuple like enum variant. This is used for types that do not implement the `Version`
///   trait. Which is basically all standard library types such as `Vec<T>`, `i32`, `u32`, etc. This will call
///   `serde_packet::from_slice_prim`
/// - no attribute and unit like enum variant. This will just return that unit variant whenever it is
///
/// The way we know which variant to target is using the `discrim` argument provided to
/// `VariantDeserializer::deserialize_variant`. See how it is used in `avatar_methods.rs` for example usage.
pub trait VariantDeserializer {
    fn deserialize_variant(
        discrim: &'static str, input: &[u8], context: &Context,
    ) -> Result<Self, PacketError>
    where
        Self: Sized;
}

pub(crate) mod prelude {
    pub(crate) use macros::{EventPrinter, Version};
    pub(crate) use serde::{Deserialize, Serialize};

    pub(crate) use super::event::{EventPrinter, EventType, PacketParser, TrackVersion, VersionInfo};
    pub(crate) use super::serde_packet::from_slice;
    pub(crate) use super::types::Vector3;
    pub(crate) use super::{Context, Packet, PacketError};
}
