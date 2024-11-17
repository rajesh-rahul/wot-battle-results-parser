use super::vehicle_descr::parse_compact_descr;
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
    pub data:        serde_json::Map<String, serde_json::Value>,
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
        let _size = stream.le_u32()?;

        let key = utils::version_as_string(context.get_version());
        let entity_type = find_entity_type(context.version, entity_type_id as usize).ok_or_else(|| {
            PacketError::NotFoundError {
                err: format!("entity type with id: {entity_type_id} not found for version: {key}"),
            }
        })?;

        let props = context
            .version_data
            .entities
            .iter()
            .find_map(|it| {
                if it.ty == entity_type {
                    Some(it.exposed_props)
                } else {
                    None
                }
            })
            .unwrap();

        let num_fields = stream.le_u8()?;
        let mut data = serde_json::Map::new();

        for _ in 0..num_fields {
            let idx = stream.le_u8()?;
            let Some(prop_data) = props.get(idx as usize) else {
                return Err(PacketError::CreateEntityParseError {
                    entity_type,
                    root_cause: format!("{idx} out of bounds for entity props"),
                });
            };

            let remaining = stream.remaining_input();
            let (remaining, mut value) = prop_data.datatype.input_to_json_value(remaining)?;

            if entity_type == EntityType::Vehicle {
                transform_vehicle_compact_description(&mut value, context.version);
            }

            stream = InputStream::from(remaining);
            data.insert(prop_data.name.to_string(), value);
        }

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
                data,
            }))
        }
    }
}

// TODO: This is pointless
fn transform_vehicle_compact_description(value: &mut serde_json::Value, version: [u16; 4]) {
    if let Some(comp_descr_val) = value.pointer_mut("/compDescr") {
        let serde_json::Value::String(comp_descr_str) = comp_descr_val.clone() else {
            return;
        };
        let Ok(comp_descr_bytes) = hex::decode(comp_descr_str) else {
            return;
        };

        let Ok(comp_desc_parsed) = parse_compact_descr(comp_descr_bytes, version) else {
            return;
        };

        *comp_descr_val = serde_json::to_value(comp_desc_parsed).unwrap();
    }
}
