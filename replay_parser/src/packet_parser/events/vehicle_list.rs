use super::vehicle_added::{parse_vehicle_data, VehicleData};
use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{decompress_vec, make_pickle_val, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct VehicleList(Vec<VehicleData>);

impl VehicleList {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8], context: &Context,
    ) -> Result<VehicleList, PacketError> {
        let value = make_pickle_val(&decompress_vec(arena_data)?)?.list()?;

        let vehicle_list = value
            .into_iter()
            .map(|veh_data| {
                veh_data
                    .list()
                    .and_then(|it| parse_vehicle_data(it, context.version))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(VehicleList(vehicle_list))
    }
}
