use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Serialize, Debug, Clone)]
pub struct ShowDamageFromExplosion {
    pub vehicle_id: i32,

    pub attacker_id: i32,
    pub center: Vector3,
    pub effects_idx: u8,
    pub damage: Option<i32>,
    pub damage_factor: u8,
    pub damage_caused_by_discrete_factor: Option<f32>,
}


impl ShowDamageFromExplosion {
    pub fn parse_from(
        gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        let res = if context.version >= [1, 26, 1, 0] {
            ShowDamageFromExplosion {
                vehicle_id: gen_info.entity_id,
                attacker_id: reader.le_i32()?,
                center: reader.vector3()?,
                effects_idx: reader.le_u8()?,
                damage: Some(reader.le_i32()?),
                damage_factor: reader.le_u8()?,
                damage_caused_by_discrete_factor: None,
            }
        } else if context.version >= [1, 26, 0, 1] {
            ShowDamageFromExplosion {
                vehicle_id: gen_info.entity_id,
                attacker_id: reader.le_i32()?,
                center: reader.vector3()?,
                effects_idx: reader.le_u8()?,
                damage: Some(reader.le_i32()?),
                damage_factor: reader.le_u8()?,
                damage_caused_by_discrete_factor: Some(reader.le_f32()?),
            }
        } else if context.version >= [1, 25, 1, 0] {
            ShowDamageFromExplosion {
                vehicle_id: gen_info.entity_id,
                attacker_id: reader.le_i32()?,
                center: reader.vector3()?,
                effects_idx: reader.le_u8()?,
                damage: Some(reader.le_i32()?),
                damage_factor: reader.le_u8()?,
                damage_caused_by_discrete_factor: None,
            }
        } else if context.version >= [1, 22, 1, 0] {
            ShowDamageFromExplosion {
                vehicle_id: gen_info.entity_id,
                attacker_id: reader.le_i32()?,
                center: reader.vector3()?,
                effects_idx: reader.le_u8()?,
                damage: None,
                damage_factor: reader.le_u8()?,
                damage_caused_by_discrete_factor: None,
            }
        } else if context.version >= [1, 22, 0, 1] {
            ShowDamageFromExplosion {
                vehicle_id: gen_info.entity_id,
                attacker_id: reader.le_i32()?,
                center: reader.vector3()?,
                effects_idx: reader.le_u8()?,
                damage: None,
                damage_factor: reader.le_u8()?,
                damage_caused_by_discrete_factor: Some(reader.le_f32()?),
            }
        } else {
            ShowDamageFromExplosion {
                vehicle_id: gen_info.entity_id,
                attacker_id: reader.le_i32()?,
                center: reader.vector3()?,
                effects_idx: reader.le_u8()?,
                damage: None,
                damage_factor: reader.le_u8()?,
                damage_caused_by_discrete_factor: None,
            }
        };


        Ok(EventType::ShowDamageFromExplosion(res))
    }
}
