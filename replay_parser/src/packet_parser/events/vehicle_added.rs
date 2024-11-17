use serde_pickle::Value as PickleVal;

use super::vehicle_descr::{parse_compact_descr, VehicleCompactDescr};
use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{decompress_vec, make_pickle_val, parse_truthy_value, parse_value, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct VehicleAdded {
    #[serde(flatten)]
    data: VehicleData,
}


impl VehicleAdded {
    pub fn parse_from(
        gen_info: EntMethodGeneralInfo, arena_data: &[u8], context: &Context,
    ) -> Result<VehicleAdded, PacketError> {
        let value = decompress_vec(arena_data).and_then(|it| make_pickle_val(&it))?;

        Ok(VehicleAdded {
            data: parse_vehicle_data(value.list()?, context.version)?,
        })
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct VehicleData {
    pub vehicle_compact_descr: Option<VehicleCompactDescr>,
    pub name:                  String,
    pub team:                  i64,
    pub is_alive:              i64,
    pub is_avatar_ready:       i64,
    pub is_team_killer:        i64,
    pub account_dbid:          i64,
    pub clan_abbrev:           String,
    pub clan_dbid:             i64,
    pub pre_battle_id:         i64,
}

pub(crate) fn parse_vehicle_data(
    vehicle_data: Vec<PickleVal>, version: [u16; 4],
) -> Result<VehicleData, PacketError> {
    let vehicle_compact_descr = if let Some(PickleVal::Bytes(compact_descr)) = vehicle_data.get(1) {
        Some(parse_compact_descr(compact_descr.clone(), version)?)
    } else {
        None
    };

    Ok(VehicleData {
        vehicle_compact_descr,
        name: parse_value(2, &vehicle_data)?,
        team: parse_value(3, &vehicle_data)?,
        is_alive: parse_truthy_value(4, &vehicle_data)?,
        is_avatar_ready: parse_truthy_value(5, &vehicle_data)?,
        is_team_killer: parse_truthy_value(6, &vehicle_data)?,
        account_dbid: parse_value(7, &vehicle_data)?,
        clan_abbrev: parse_value(8, &vehicle_data)?,
        clan_dbid: parse_value(9, &vehicle_data)?,
        pre_battle_id: parse_value(10, &vehicle_data)?,
    })
}
