use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize, Version)]
pub struct FogOfWar {
    pub is_enabled:          bool,
    pub has_hidden_vehicles: bool,
}

use super::EntMethodGeneralInfo;
use crate::utils::{make_pickle_val, PickleOps};


impl FogOfWar {
    pub fn parse_from(_gen_info: EntMethodGeneralInfo, arena_data: &[u8]) -> Result<FogOfWar, PacketError> {
        let data: i64 = make_pickle_val(arena_data)?.deser()?;


        Ok(FogOfWar {
            is_enabled:          data & 1 != 0,
            has_hidden_vehicles: data & 2 != 0,
        })
    }
}
