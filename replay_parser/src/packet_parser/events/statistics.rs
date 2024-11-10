use super::{EntMethodGeneralInfo, VehicleStatistics};
use crate::packet_parser::prelude::*;
use crate::utils::{decompress_vec, make_pickle_val, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct Statistics(Vec<VehicleStatistics>);

impl Statistics {
    pub fn parse_from(_gen_info: EntMethodGeneralInfo, arena_data: &[u8]) -> Result<Statistics, PacketError> {
        let data_list = make_pickle_val(&decompress_vec(arena_data)?)?.list()?;

        let result = data_list
            .into_iter()
            .map(|it| {
                it.deser()
                    .and_then(|(vehicle_id, frags)| Ok(VehicleStatistics { vehicle_id, frags }))
            })
            .collect::<Result<_, _>>();

        Ok(Statistics(result?))
    }
}
