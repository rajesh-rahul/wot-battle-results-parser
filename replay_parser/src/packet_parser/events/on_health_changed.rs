use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Serialize, Debug, Clone)]
pub struct OnHealthChanged {
    pub vehicle_id: i32,

    pub new_health:             i16,
    pub old_health:             Option<i16>,
    pub attacker_id:            i32,
    pub attack_reason:          u8,
    pub attack_reason_extra_id: Option<i8>,
}


impl OnHealthChanged {
    pub fn parse_from(
        gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        let res = if context.version >= [1, 26, 1, 0] {
            OnHealthChanged {
                vehicle_id:             gen_info.entity_id,
                new_health:             reader.le_i16()?,
                old_health:             Some(reader.le_i16()?),
                attacker_id:            reader.le_i32()?,
                attack_reason:          reader.le_u8()?,
                attack_reason_extra_id: Some(reader.le_i8()?),
            }
        } else if context.version >= [1, 11, 1, 0] {
            OnHealthChanged {
                vehicle_id:             gen_info.entity_id,
                new_health:             reader.le_i16()?,
                old_health:             Some(reader.le_i16()?),
                attacker_id:            reader.le_i32()?,
                attack_reason:          reader.le_u8()?,
                attack_reason_extra_id: None,
            }
        } else {
            OnHealthChanged {
                vehicle_id:             gen_info.entity_id,
                new_health:             reader.le_i16()?,
                old_health:             None,
                attacker_id:            reader.le_i32()?,
                attack_reason:          reader.le_u8()?,
                attack_reason_extra_id: None,
            }
        };

        Ok(EventType::OnHealthChanged(res))
    }
}
