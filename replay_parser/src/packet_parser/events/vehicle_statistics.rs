use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{decompress_vec, make_pickle_val, PickleOps};

#[derive(Debug, Clone, Serialize, Version)]
pub struct VehicleStatistics {
    pub vehicle_id: i32,
    pub frags:      i32,
}

impl VehicleStatistics {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8],
    ) -> Result<VehicleStatistics, PacketError> {
        let data: (i32, i32) = make_pickle_val(&decompress_vec(arena_data)?)?.deser()?;
        let (vehicle_id, frags) = data;

        Ok(VehicleStatistics { vehicle_id, frags })
    }
}
