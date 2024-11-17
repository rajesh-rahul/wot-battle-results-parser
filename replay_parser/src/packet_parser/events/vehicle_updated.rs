use super::vehicle_added::{parse_vehicle_data, VehicleData};
use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{decompress_vec, make_pickle_val, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct VehicleUpdated(VehicleData);

impl VehicleUpdated {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8], context: &Context,
    ) -> Result<VehicleUpdated, PacketError> {
        let value = make_pickle_val(&decompress_vec(arena_data)?)?.list()?;

        Ok(VehicleUpdated(parse_vehicle_data(value, context.version)?))
    }
}
