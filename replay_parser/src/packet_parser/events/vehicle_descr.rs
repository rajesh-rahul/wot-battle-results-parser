use serde::Serialize;

use super::EntMethodGeneralInfo;
use crate::utils::{make_pickle_val, PickleOps};
use crate::PacketError;

#[derive(Debug, Clone, Serialize)]
pub struct VehicleDescr {
    pub vehicle_id:    i32,
    pub compact_descr: VehicleCompactDescr,
    pub max_health:    i32,
}


impl VehicleDescr {
    // NOTE: Picking the values of the picle value ourselves is faster (like .list())
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8],
    ) -> Result<VehicleDescr, PacketError> {
        let data: (i32, Vec<u8>, i32) = make_pickle_val(arena_data)?.deser()?;
        let (vehicle_id, compact_descr_bytes, max_health) = data;

        Ok(VehicleDescr {
            vehicle_id,
            compact_descr: parse_compact_descr(compact_descr_bytes)?,
            max_health,
        })
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct VehicleCompactDescr {
    pub nation_id:             u8,
    pub vehicle_type_id:       u8,
    pub components:            Vec<u8>,
    pub optional_device_slots: i32,
    pub optional_devices:      Vec<u8>,
}


pub fn parse_compact_descr(compact_descr: Vec<u8>) -> Result<VehicleCompactDescr, PacketError> {
    let header = compact_descr[0];

    if header & 15 != 1 {
        return Err(PacketError::Misc {
            err: format!("Unable to parse vehicle compact description"),
        });
    }

    let nation_id = header >> 4 & 15;
    let vehicle_type_id = compact_descr[1];

    let mut idx = 10 + (1) * 4;
    let components = compact_descr[2..idx].to_vec();

    let flags = compact_descr[idx];
    idx += 1;

    let mut count = 0;
    let mut optional_device_slots = 0;

    for i in 0..3 {
        if (flags & 1 << i) != 0 {
            count += 1;
            optional_device_slots |= 1 << i;
        }
    }

    let optional_devices = compact_descr[idx..(idx + count * 2)].to_vec();

    if optional_devices.len() % 2 != 0 {
        return Err(PacketError::Misc {
            err: format!("Unable to parse vehicle compact description"),
        });
    }

    Ok(VehicleCompactDescr {
        nation_id,
        vehicle_type_id,
        components,
        optional_device_slots,
        optional_devices,
    })
}
