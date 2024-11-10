use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{make_pickle_val, parse_value, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct VehicleKilled {
    pub victim_id:     i32,
    pub killer_id:     i32,
    pub equipment_id:  i32,
    pub attack_reason: i32, // use attack reason enum (but it differs by version)

    // #[version([1, 17, 0, 0])]
    pub num_vehicles_affected: Option<i32>,
}

impl VehicleKilled {
    pub fn parse_from(
        gen_info: EntMethodGeneralInfo, arena_data: &[u8],
    ) -> Result<VehicleKilled, PacketError> {
        let value = make_pickle_val(arena_data)?.list()?;


        let vehicle_killed = VehicleKilled {
            victim_id:             parse_value(0, &value)?,
            killer_id:             parse_value(1, &value)?,
            equipment_id:          parse_value(2, &value)?,
            attack_reason:         parse_value(3, &value)?,
            num_vehicles_affected: if value.len() > 4 {
                parse_value(4, &value)?
            } else {
                None
            },
        };

        Ok(vehicle_killed)
    }
}
