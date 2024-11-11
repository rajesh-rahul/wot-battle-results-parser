use wot_types::WotValue;

use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{decompress_vec, make_pickle_val};

#[derive(Debug, Clone, Serialize)]
pub struct ViewPoints {
    data: WotValue,
}

impl ViewPoints {
    pub fn parse_from(_gen_info: EntMethodGeneralInfo, arena_data: &[u8]) -> Result<ViewPoints, PacketError> {
        let data = make_pickle_val(&decompress_vec(arena_data)?)?;

        Ok(ViewPoints {
            data: serde_pickle::from_value(data)?,
        })
    }
}
