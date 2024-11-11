use events::{
    OnHealthChanged, OnStaticCollision, ShowDamageFromExplosion, ShowDamageFromShot, ShowShooting,
    ShowTracer, UpdatePositions, UpdateVehicleAmmo,
};
use macros::EventPrinter;
// pub use avatar_methods::update_arena::*;
use packet_parser::update_arena::parse_update_arena_method;
use serde::Serialize;
use wot_data::{AvatarMethod, EntityType, VehicleMethod, AVATAR_METHODS, VEHICLE_METHODS};

use crate::packet_parser::InputStream;
use crate::*;
// pub use vehicle_methods::*;

// use self::avatar_methods::AvatarMethods;


/// Represents all packets of type `0x08`. `0x08` packet seems to describe a method call on an entity.
/// Refers to multiple types of events (see `EntityMethod` enum)
#[derive(Debug, Clone, EventPrinter, Serialize)]
pub struct EntMethodGeneralInfo {
    /// ID of the entity who called this method
    #[event_debug(as_player)]
    pub entity_id:   i32,
    pub entity_type: EntityType,
    pub method_name: String,
}

pub(crate) fn parse_entity_method(packet: &Packet, context: &Context) -> Result<EventType, PacketError> {
    let mut reader = InputStream::from(packet.payload());

    let entity_id = reader.le_i32()?;
    let method_id = reader.le_i32()?;
    let _size = reader.le_u32()?;
    let entity_type = context.find_entity_type(entity_id)?;

    let payload = reader.remaining_input();

    match entity_type {
        EntityType::Avatar => {
            let method_name = find_method(context.version, AVATAR_METHODS, method_id as usize)?;
            let gen_info = EntMethodGeneralInfo::from(entity_id, entity_type, method_name);

            parse_avatar_methods(gen_info, method_name, context, payload)
        }
        EntityType::Vehicle => {
            let method_name = find_method(context.version, VEHICLE_METHODS, method_id as usize)?;
            let gen_info = EntMethodGeneralInfo::from(entity_id, entity_type, method_name);

            parse_vehicle_methods(gen_info, method_name, context, payload)
        }
        _ => return Err(PacketError::UnsupportedEntity { entity_type }),
    }
}

fn parse_avatar_methods(
    gen_info: EntMethodGeneralInfo, method_name: AvatarMethod, context: &Context, payload: &[u8],
) -> Result<EventType, PacketError> {
    match method_name {
        AvatarMethod::UpdateArena => parse_update_arena_method(gen_info, payload, context),
        AvatarMethod::UpdateVehicleAmmo => UpdateVehicleAmmo::parse_from(gen_info, payload, context),
        AvatarMethod::UpdatePositions => UpdatePositions::parse_from(gen_info, payload, context),
        AvatarMethod::ShowTracer => ShowTracer::parse_from(gen_info, payload, context),
        _ => Ok(EventType::UnimplementedEntityMethod(gen_info)),
    }
}

fn parse_vehicle_methods(
    gen_info: EntMethodGeneralInfo, method_name: VehicleMethod, context: &Context, payload: &[u8],
) -> Result<EventType, PacketError> {
    match method_name {
        VehicleMethod::OnHealthChanged => OnHealthChanged::parse_from(gen_info, payload, context),
        VehicleMethod::OnStaticCollision => OnStaticCollision::parse_from(gen_info, payload, context),
        VehicleMethod::ShowDamageFromExplosion => {
            ShowDamageFromExplosion::parse_from(gen_info, payload, context)
        }
        VehicleMethod::ShowDamageFromShot => ShowDamageFromShot::parse_from(gen_info, payload, context),
        VehicleMethod::ShowShooting => ShowShooting::parse_from(gen_info, payload, context),
        _ => Ok(EventType::UnimplementedEntityMethod(gen_info)),
    }
}


fn find_method<T: Clone + Copy>(
    version: [u16; 4], map: phf::Map<&'static str, &'static [T]>, method_id: usize,
) -> Result<T, PacketError> {
    let version_str = utils::version_as_string(version);

    let version_major_str = || {
        let mut v = version;
        v[3] = 0;

        utils::version_as_string(v)
    };

    map.get(&version_str)
        .or_else(|| map.get(&version_major_str()))
        .map(|method_arr| method_arr.get(method_id))
        .flatten()
        .copied()
        .ok_or_else(|| PacketError::NotFoundError {
            err: format!("{method_id} not found"),
        })
}

impl EntMethodGeneralInfo {
    fn from<T: ToString>(entity_id: i32, entity_type: EntityType, method_name: T) -> EntMethodGeneralInfo {
        EntMethodGeneralInfo {
            entity_id,
            entity_type,
            method_name: method_name.to_string(),
        }
    }
}
