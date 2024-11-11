use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;
use crate::utils::find_entity_type;
use crate::wot_data::EntityType;

#[derive(Debug, Clone, EventPrinter, Version, Serialize)]
pub struct CreateEntity {
    pub entity_id:   i32,
    pub entity_type: EntityType,
    pub vehicle_id:  Option<i32>,
    pub space_id:    i32,
    pub unknown:     i32,
    pub position:    Vector3,
    pub direction:   Vector3,
}

impl PacketParser for CreateEntity {
    fn parse_mut(packet: &Packet, context: &mut Context) -> Result<EventType, PacketError> {
        let mut stream = InputStream::from(packet.payload());

        let entity_id = stream.le_i32()?;
        let entity_type_id = stream.le_u16()?;

        let vehicle_id = if context.version >= [0, 9, 14, 0] {
            Some(stream.le_i32()?)
        } else {
            None
        };

        let space_id = stream.le_i32()?;
        let unknown = stream.le_i32()?;
        let position = stream.vector3()?;
        let direction = stream.vector3()?;
        let size = stream.le_u32()?;

        let _data = stream.take(size as usize)?; // TODO: We will ignore the data for now...


        let key = utils::version_as_string(context.get_version());
        // let version = utils::version_as_string(context.get_version());
        let entity_type = find_entity_type(context.version, entity_type_id as usize).ok_or_else(|| {
            PacketError::NotFoundError {
                err: format!("entity type with id: {entity_type_id} not found for version: {key}"),
            }
        })?;

        context.add_entity(entity_id, entity_type);

        if !stream.remaining_input().is_empty() {
            Err(PacketError::UnconsumedInput)
        } else {
            Ok(EventType::CreateEntity(CreateEntity {
                entity_id,
                entity_type,
                vehicle_id,
                space_id,
                unknown,
                position,
                direction,
            }))
        }
    }
}
