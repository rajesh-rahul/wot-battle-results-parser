use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Serialize, Debug, Clone)]
pub struct ShowDamageFromShot {
    pub vehicle_id: i32,

    pub attacker_vehicle_id: i32,
    pub points: Vec<u64>,
    pub effects_index: u8,
    pub damage: Option<i32>,
    pub damage_factor: u8,
    pub last_material_shield: Option<u8>,
    pub damage_caused_by_discrete_factor: Option<f32>,
}


impl ShowDamageFromShot {
    pub fn parse_from(
        gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        let res = if context.version >= [1, 26, 1, 0] {
            ShowDamageFromShot {
                vehicle_id: gen_info.entity_id,
                attacker_vehicle_id: reader.le_i32()?,
                points: reader.array(InputStream::le_u64)?,
                effects_index: reader.le_u8()?,
                damage: Some(reader.le_i32()?),
                damage_factor: reader.le_u8()?,
                last_material_shield: Some(reader.le_u8()?),
                damage_caused_by_discrete_factor: None,
            }
        } else if context.version >= [1, 26, 0, 1] {
            ShowDamageFromShot {
                vehicle_id: gen_info.entity_id,
                attacker_vehicle_id: reader.le_i32()?,
                points: reader.array(InputStream::le_u64)?,
                effects_index: reader.le_u8()?,
                damage: Some(reader.le_i32()?),
                damage_factor: reader.le_u8()?,
                last_material_shield: Some(reader.le_u8()?),
                damage_caused_by_discrete_factor: Some(reader.le_f32()?),
            }
        } else if context.version >= [1, 25, 1, 0] {
            ShowDamageFromShot {
                vehicle_id: gen_info.entity_id,
                attacker_vehicle_id: reader.le_i32()?,
                points: reader.array(InputStream::le_u64)?,
                effects_index: reader.le_u8()?,
                damage: Some(reader.le_i32()?),
                damage_factor: reader.le_u8()?,
                last_material_shield: Some(reader.le_u8()?),
                damage_caused_by_discrete_factor: None,
            }
        } else if context.version >= [1, 22, 1, 0] {
            ShowDamageFromShot {
                vehicle_id: gen_info.entity_id,
                attacker_vehicle_id: reader.le_i32()?,
                points: reader.array(InputStream::le_u64)?,
                effects_index: reader.le_u8()?,
                damage: None,
                damage_factor: reader.le_u8()?,
                last_material_shield: Some(reader.le_u8()?),
                damage_caused_by_discrete_factor: None,
            }
        } else if context.version >= [1, 22, 0, 1] {
            ShowDamageFromShot {
                vehicle_id: gen_info.entity_id,
                attacker_vehicle_id: reader.le_i32()?,
                points: reader.array(InputStream::le_u64)?,
                effects_index: reader.le_u8()?,
                damage: None,
                damage_factor: reader.le_u8()?,
                last_material_shield: Some(reader.le_u8()?),
                damage_caused_by_discrete_factor: Some(reader.le_f32()?),
            }
        } else if context.version >= [1, 13, 0, 0] {
            ShowDamageFromShot {
                vehicle_id: gen_info.entity_id,
                attacker_vehicle_id: reader.le_i32()?,
                points: reader.array(InputStream::le_u64)?,
                effects_index: reader.le_u8()?,
                damage: None,
                damage_factor: reader.le_u8()?,
                last_material_shield: Some(reader.le_u8()?),
                damage_caused_by_discrete_factor: None,
            }
        } else {
            ShowDamageFromShot {
                vehicle_id: gen_info.entity_id,
                attacker_vehicle_id: reader.le_i32()?,
                points: reader.array(InputStream::le_u64)?,
                effects_index: reader.le_u8()?,
                damage: None,
                damage_factor: reader.le_u8()?,
                last_material_shield: None,
                damage_caused_by_discrete_factor: None,
            }
        };


        Ok(EventType::ShowDamageFromShot(res))
    }
}
