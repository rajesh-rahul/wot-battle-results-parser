use serde::de;

use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;
use crate::utils;
use crate::utils::find_entity_type;
use crate::wot_types::WotValue;

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct CreateAvatar {
    pub entity_id: i32,
    pub data:      serde_json::Map<String, serde_json::Value>,
    pub unknown:   Vec<u8>,
}


impl PacketParser for CreateAvatar {
    fn parse_mut(packet: &Packet, context: &mut Context) -> Result<BattleEvent, PacketError> {
        let mut reader = InputStream::from(packet.payload());

        let entity_id = reader.le_i32()?;
        let entity_type_id = reader.le_u16()?;

        if context.version >= [0, 9, 14, 0] {
            // not exactly sure what this is
            let _padding = reader.le_u32()?;
        }
        // TODO: Add a check to ensure size value make sense
        let _size = reader.le_u32()?;

        let field_data = context.version_data.special_formats.create_avatar;

        let mut map = serde_json::Map::new();
        let mut remaining = reader.remaining_input();
        for field in field_data {
            let key = field.name.to_string();
            let (r, value) = field.datatype.input_to_json_value(remaining)?;
            remaining = r;

            map.insert(key, value);
        }

        let key = utils::version_as_string(context.version);
        let entity_type = find_entity_type(context.version, entity_type_id as usize).ok_or_else(|| {
            PacketError::NotFoundError {
                err: format!("entity type with id: {entity_type_id} not found for version: {key}"),
            }
        })?;

        context.add_entity(entity_id, entity_type);

        Ok(BattleEvent::CreateAvatar(CreateAvatar {
            entity_id,
            data: map,
            unknown: remaining.to_vec(),
        }))
    }
}

impl CreateAvatar {
    pub fn arena_unique_id(&self) -> Option<u64> {
        self.data.get("arenaUniqueID").map(|it| it.as_u64()).flatten()
    }
}


fn deserialize_pickle<'de, D>(deserializer: D) -> core::result::Result<WotValue, D::Error>
where
    D: de::Deserializer<'de>,
{
    let vec: &[u8] = de::Deserialize::deserialize(deserializer)?;
    let pickle =
        serde_pickle::value_from_slice(vec, serde_pickle::DeOptions::new().replace_unresolved_globals())
            .unwrap();

    let wot_value = serde_pickle::from_value(pickle).map_err(|e| serde::de::Error::custom(e.to_string()))?;
    Ok(wot_value)
}
