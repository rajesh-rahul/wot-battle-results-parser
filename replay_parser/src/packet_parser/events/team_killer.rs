use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{make_pickle_val, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct TeamKiller {
    vehicle_id: i32,
    status:     Option<i32>,
}

impl TeamKiller {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8], context: &Context,
    ) -> Result<TeamKiller, PacketError> {
        if context.version > [1, 0, 0, 0] {
            let data: (i32, i32) = make_pickle_val(arena_data)?.deser()?;
            let (vehicle_id, status) = data;

            Ok(TeamKiller {
                vehicle_id,
                status: Some(status),
            })
        } else {
            Ok(TeamKiller {
                vehicle_id: make_pickle_val(arena_data)?.deser()?,
                status:     None,
            })
        }
    }
}
