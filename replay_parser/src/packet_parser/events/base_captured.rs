use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{make_pickle_val, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct BaseCaptured {
    pub team:    i32,
    pub base_id: i32,
}

impl BaseCaptured {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8],
    ) -> Result<BaseCaptured, PacketError> {
        let data: (i32, i32) = make_pickle_val(arena_data)?.deser()?;
        let (team, base_id) = data;

        Ok(BaseCaptured { team, base_id })
    }
}
