use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Serialize, Debug, Clone)]
pub struct OnStaticCollision {
    pub vehicle_id: i32,

    pub energy:             f32,
    pub point:              Vector3,
    pub normal:             Vector3,
    pub misc_flags:         u8,
    pub damage:             Option<f32>,
    pub damage_left_track:  Option<f32>,
    pub damage_right_track: Option<f32>,
    pub destr_effects_idx:  Option<i8>,
    pub material_kind:      Option<u8>, // this value is there but not useful
    pub destr_max_health:   Option<u16>,
}


impl OnStaticCollision {
    pub fn parse_from(
        gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        let res = if context.version >= [1, 3, 0, 0] {
            OnStaticCollision {
                vehicle_id:         gen_info.entity_id,
                energy:             reader.le_f32()?,
                point:              reader.vector3()?,
                normal:             reader.vector3()?,
                misc_flags:         reader.le_u8()?,
                damage:             Some(reader.le_f32()?),
                damage_left_track:  None,
                damage_right_track: None,
                destr_effects_idx:  Some(reader.le_i8()?),
                material_kind:      None,
                destr_max_health:   Some(reader.le_u16()?),
            }
        } else if context.version >= [0, 9, 23, 0] {
            OnStaticCollision {
                vehicle_id:         gen_info.entity_id,
                energy:             reader.le_f32()?,
                point:              reader.vector3()?,
                normal:             reader.vector3()?,
                misc_flags:         reader.le_u8()?,
                damage:             Some(reader.le_f32()?),
                damage_left_track:  Some(reader.le_f32()?),
                damage_right_track: Some(reader.le_f32()?),
                destr_effects_idx:  Some(reader.le_i8()?),
                material_kind:      None,
                destr_max_health:   Some(reader.le_u16()?),
            }
        } else if context.version >= [0, 9, 17, 0] {
            OnStaticCollision {
                vehicle_id:         gen_info.entity_id,
                energy:             reader.le_f32()?,
                point:              reader.vector3()?,
                normal:             reader.vector3()?,
                misc_flags:         reader.le_u8()?,
                damage:             Some(reader.le_f32()?),
                damage_left_track:  Some(reader.le_f32()?),
                damage_right_track: Some(reader.le_f32()?),
                destr_effects_idx:  Some(reader.le_i8()?),
                material_kind:      Some(reader.le_u8()?),
                destr_max_health:   None,
            }
        } else if context.version >= [0, 9, 16, 0] {
            OnStaticCollision {
                vehicle_id:         gen_info.entity_id,
                energy:             reader.le_f32()?,
                point:              reader.vector3()?,
                normal:             reader.vector3()?,
                misc_flags:         reader.le_u8()?,
                damage:             Some(reader.le_f32()?),
                damage_left_track:  Some(reader.le_f32()?),
                damage_right_track: Some(reader.le_f32()?),
                destr_effects_idx:  None,
                material_kind:      None,
                destr_max_health:   None,
            }
        } else {
            OnStaticCollision {
                vehicle_id:         gen_info.entity_id,
                energy:             reader.le_f32()?,
                point:              reader.vector3()?,
                normal:             reader.vector3()?,
                misc_flags:         reader.le_u8()?,
                damage:             None,
                damage_left_track:  None,
                damage_right_track: None,
                destr_effects_idx:  None,
                material_kind:      None,
                destr_max_health:   None,
            }
        };


        Ok(EventType::OnStaticCollision(res))
    }
}
