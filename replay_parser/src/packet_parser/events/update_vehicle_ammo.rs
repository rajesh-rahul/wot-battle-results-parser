use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Debug, Clone, Serialize)]
pub struct UpdateVehicleAmmo {
    vehicle_id: Option<i32>,
    compact_descr: i32,
    quantity: u16,
    quantity_in_clip_or_next_stage: u8,
    time_remaining: serde_json::Value,
    // Only appears in 0.9.20
    base_time: Option<f32>,
    index: serde_json::Value,
    total_time: Option<i16>,
    previous_stage: Option<u8>,
}


impl UpdateVehicleAmmo {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        let res = if context.version < [0, 9, 17, 0] {
            UpdateVehicleAmmo {
                compact_descr: reader.le_i32()?,
                quantity: reader.le_u16()?,
                quantity_in_clip_or_next_stage: reader.le_u8()?,
                time_remaining: serde_json::to_value(reader.le_i16()?).unwrap(),
                vehicle_id: None,
                base_time: None,
                index: serde_json::Value::Null,
                previous_stage: None,
                total_time: None,
            }
        } else if context.version >= [0, 9, 17, 0] {
            UpdateVehicleAmmo {
                vehicle_id: Some(reader.le_i32()?),
                compact_descr: reader.le_i32()?,
                quantity: reader.le_u16()?,
                quantity_in_clip_or_next_stage: reader.le_u8()?,
                time_remaining: serde_json::to_value(reader.le_i16()?).unwrap(),
                base_time: None,
                index: serde_json::Value::Null,
                previous_stage: None,
                total_time: None,
            }
        } else if context.version >= [0, 9, 20, 0] {
            UpdateVehicleAmmo {
                vehicle_id: Some(reader.le_i32()?),
                compact_descr: reader.le_i32()?,
                quantity: reader.le_u16()?,
                quantity_in_clip_or_next_stage: reader.le_u8()?,
                time_remaining: serde_json::to_value(reader.le_f32()?).unwrap(),
                base_time: Some(reader.le_f32()?),
                index: serde_json::to_value(reader.le_u8()?).unwrap(),
                total_time: None,
                previous_stage: None,
            }
        } else if context.version >= [0, 9, 21, 0] {
            UpdateVehicleAmmo {
                vehicle_id: Some(reader.le_i32()?),
                compact_descr: reader.le_i32()?,
                quantity: reader.le_u16()?,
                quantity_in_clip_or_next_stage: reader.le_u8()?,
                time_remaining: serde_json::to_value(reader.le_i16()?).unwrap(),
                base_time: None,
                index: serde_json::Value::Null,
                previous_stage: None,
                total_time: None,
            }
        } else if context.version >= [1, 0, 1, 0] {
            UpdateVehicleAmmo {
                vehicle_id: Some(reader.le_i32()?),
                compact_descr: reader.le_i32()?,
                quantity: reader.le_u16()?,
                quantity_in_clip_or_next_stage: reader.le_u8()?,
                time_remaining: serde_json::to_value(reader.le_i16()?).unwrap(),
                total_time: Some(reader.le_i16()?),
                base_time: None,
                previous_stage: None,
                index: serde_json::Value::Null,
            }
        } else if context.version >= [1, 10, 0, 0] {
            UpdateVehicleAmmo {
                vehicle_id: Some(reader.le_i32()?),
                compact_descr: reader.le_i32()?,
                quantity: reader.le_u16()?,
                quantity_in_clip_or_next_stage: reader.le_u8()?,
                previous_stage: Some(reader.le_u8()?),
                time_remaining: serde_json::to_value(reader.le_i16()?).unwrap(),
                total_time: Some(reader.le_i16()?),
                base_time: None,
                index: serde_json::Value::Null,
            }
        } else if context.version >= [1, 21, 1, 0] {
            UpdateVehicleAmmo {
                vehicle_id: Some(reader.le_i32()?),
                compact_descr: reader.le_i32()?,
                quantity: reader.le_u16()?,
                quantity_in_clip_or_next_stage: reader.le_u8()?,
                previous_stage: Some(reader.le_u8()?),
                time_remaining: serde_json::to_value(reader.le_i16()?).unwrap(),
                total_time: Some(reader.le_i16()?),
                base_time: None,
                index: serde_json::to_value(reader.le_i16()?).unwrap(),
            }
        } else {
            unreachable!()
        };

        Ok(EventType::UpdateVehicleAmmo(res))
    }
}
