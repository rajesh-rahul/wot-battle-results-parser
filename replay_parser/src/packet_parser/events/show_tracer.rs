use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Serialize, Debug, Clone)]
pub struct ShowTracer {
    pub shooter_id:      i32,
    pub shot_id:         i32,
    pub is_ricochet:     u8,
    pub effects_index:   u8,
    pub ref_start_point: Vector3,
    pub velocity:        Vector3,
    pub gravity:         f32,
    pub max_shot_dist:   f32,
    pub gun_index:       Option<u8>,
}


impl ShowTracer {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        let res = if context.version >= [1, 6, 1, 0] {
            ShowTracer {
                shooter_id:      reader.le_i32()?,
                shot_id:         reader.le_i32()?,
                is_ricochet:     reader.le_u8()?,
                effects_index:   reader.le_u8()?,
                ref_start_point: reader.vector3()?,
                velocity:        reader.vector3()?,
                gravity:         reader.le_f32()?,
                max_shot_dist:   reader.le_f32()?,
                gun_index:       Some(reader.le_u8()?),
            }
        } else if context.version >= [0, 9, 21, 0] {
            ShowTracer {
                shooter_id:      reader.le_i32()?,
                shot_id:         reader.le_i32()?,
                is_ricochet:     reader.le_u8()?,
                effects_index:   reader.le_u8()?,
                ref_start_point: reader.vector3()?,
                velocity:        reader.vector3()?,
                gravity:         reader.le_f32()?,
                max_shot_dist:   reader.le_f32()?,
                gun_index:       None,
            }
        } else if context.version >= [0, 9, 20, 0] {
            ShowTracer {
                shooter_id:      reader.le_i32()?,
                shot_id:         reader.le_i32()?,
                is_ricochet:     reader.le_u8()?,
                effects_index:   reader.le_u8()?,
                ref_start_point: reader.vector3()?,
                velocity:        reader.vector3()?,
                gravity:         reader.le_f32()?,
                max_shot_dist:   reader.le_f32()?,
                gun_index:       Some(reader.le_u8()?),
            }
        } else if context.version >= [0, 9, 16, 0] {
            ShowTracer {
                shooter_id:      reader.le_i32()?,
                shot_id:         reader.le_i32()?,
                is_ricochet:     reader.le_u8()?,
                effects_index:   reader.le_u8()?,
                ref_start_point: reader.vector3()?,
                velocity:        reader.vector3()?,
                gravity:         reader.le_f32()?,
                max_shot_dist:   reader.le_f32()?,
                gun_index:       None,
            }
        } else if context.version >= [0, 9, 15, 0] {
            ShowTracer {
                shooter_id:      reader.le_i32()?,
                shot_id:         reader.le_i32()?,
                is_ricochet:     reader.le_u8()?,
                effects_index:   reader.le_u8()?,
                ref_start_point: reader.vector3()?,
                velocity:        reader.vector3()?,
                gravity:         reader.le_f32()?,
                max_shot_dist:   reader.le_f32()?,
                gun_index:       Some(reader.le_u8()?),
            }
        } else {
            ShowTracer {
                shooter_id:      reader.le_i32()?,
                shot_id:         reader.le_i32()?,
                is_ricochet:     reader.le_u8()?,
                effects_index:   reader.le_u8()?,
                ref_start_point: reader.vector3()?,
                velocity:        reader.vector3()?,
                gravity:         reader.le_f32()?,
                max_shot_dist:   reader.le_f32()?,
                gun_index:       None,
            }
        };


        Ok(EventType::ShowTracer(res))
    }
}
