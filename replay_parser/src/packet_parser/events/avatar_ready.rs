use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{make_pickle_val, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct AvatarReady {
    vehicle_id: i32,
}

impl AvatarReady {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8],
    ) -> Result<AvatarReady, PacketError> {
        Ok(AvatarReady {
            vehicle_id: make_pickle_val(arena_data)?.deser()?,
        })
    }
}
