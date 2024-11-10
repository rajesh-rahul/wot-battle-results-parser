// pub(crate) mod avatar_methods;
// pub(crate) mod vehicle_methods;
// mod vehicle_misc_status;

use macros::EventPrinter;
// pub use avatar_methods::update_arena::*;
use nom::number::complete::le_i32;
use serde::Serialize;
use wot_data::{EntityType, AVATAR_METHODS, VEHICLE_METHODS};
use crate::{packet_parser::InputReader, *};
// pub use vehicle_methods::*;

// use self::avatar_methods::AvatarMethods;


/// Represents all packets of type `0x08`. `0x08` packet seems to describe a method call on an entity.
/// Refers to multiple types of events (see `EntityMethod` enum)
#[derive(Debug, Clone, EventPrinter, Serialize)]
pub struct EntityMethod<T: Serialize> {
    /// ID of the entity who called this method
    #[event_debug(as_player)]
    pub entity_id:   i32,
    pub entity_type: EntityType,
    pub method_name: String,
    pub data: T
    // pub method_args: Vec<serde_json::Value>, // pub method: EntityMethod,
}

impl <T: Serialize>PacketParser for EntityMethod<T> {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent, PacketError> {
        let mut reader = InputReader::from(packet.payload());

        let entity_id = reader.le_i32()?;
        let method_id = reader.le_i32()?;
        let _size = reader.le_u32()?;
        let entity_type = context.find_entity_type(entity_id)?;

        let method_name = match entity_type {
            EntityType::Avatar => get_method(context.version, AVATAR_METHODS, method_id as usize).map(|it| it.to_string()),
            EntityType::Vehicle => get_method(context.version, VEHICLE_METHODS, method_id as usize).map(|it| it.to_string()),
            _ => {
                return Err(PacketError::UnsupportedEntity { entity_type })
            }
        };

        let Some(method_name) = method_name else {
            return Err(PacketError::NotFoundError { err: "method not found".to_string() })
        };
        // let Some(entity) = context
        //     .version_data
        //     .entities
        //     .iter()
        //     .find(|it| it.ty == entity_type)
        // else {
        //     return Err(PacketError::NotFoundError {
        //         err: format!("{entity_type} not found for version {:?}", context.version),
        //     });
        // };

        // let Some(method) = entity.exposed_methods.get(method_id as usize) else {
        //     return Err(PacketError::NotFoundError {
        //         err: format!("method not found, entity = {entity_type}, method_id = {method_id}"),
        //     });
        // };

        // let mut method_args = Vec::new();
        // let mut remaining = reader.remaining_input();

        // for arg_data in method.args {
        //     let (r, arg) = arg_data.input_to_json_value(remaining)?;
        //     remaining = r;

        //     method_args.push(arg);
        // }

        // use EntityType::*;
        // let method = match entity_type {
        //     Vehicle => VehicleMethods::parse(method_data, method_id, context),
        //     Avatar => AvatarMethods::parse(method_data, method_id, context),
        //     _ => Ok(EntityMethod::NotImplemented {
        //         entity_type,
        //         method_id,
        //     }),
        // }?;

        Ok(BattleEvent::EntityMethod(EntityMethod {
            entity_id,
            entity_type,
            method_name,
        }))
    }
}


fn get_method<T: Clone + Copy>(version: [u16; 4], map: phf::Map<&'static str, &'static[T]>, method_id: usize) -> Option<T> {
    let version_str = utils::version_as_string(version);

    let version_major_str = || {
        let mut v = version;
        v[3] = 0;

        utils::version_as_string(v)
    };

    map
        .get(&version_str)
        .or_else(|| map.get(&version_major_str()))
        .map(|method_arr| method_arr.get(method_id))
        .flatten()
        .copied()
}
// impl EntityMethodEvent {
//     /// Whether we understand the entity method
//     pub fn is_unknown(&self) -> bool {
//         matches!(self.method, EntityMethod::NotImplemented { .. })
//     }
// }

// impl From<EntityMethodEvent> for EntityMethod {
//     fn from(val: EntityMethodEvent) -> Self {
//         val.method
//     }
// }

// /// Enumerates all possible entity method calls (hopefully). This enum groups methods by its entity type.
// #[derive(Debug, Clone, Serialize)]
// #[non_exhaustive]
// pub enum EntityMethod {
//     NotImplemented {
//         entity_type: EntityType,
//         method_id:   usize,
//     },
//     Vehicle(VehicleMethods),
//     Avatar(AvatarMethods),
// }

// impl EntityMethod {
//     pub fn new(
//         entity_type: EntityType, input: &[u8], context: &Context, method_id: usize,
//     ) -> std::result::Result<Self, PacketError> {
//         use EntityType::*;

//         match entity_type {
//             Vehicle => VehicleMethods::parse(input, method_id, context),
//             Avatar => AvatarMethods::parse(input, method_id, context),
//             _ => Ok(EntityMethod::NotImplemented {
//                 entity_type,
//                 method_id,
//             }),
//         }
//     }
// }


// pub trait MethodParser {
//     fn parse(input: &[u8], method_id: usize, context: &Context) -> Result<super::EntityMethod, PacketError>
//     where
//         Self: Sized;
// }
