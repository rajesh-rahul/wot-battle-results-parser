use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{make_pickle_val, parse_truthy_value, parse_value, PickleOps};

#[derive(Debug, Clone, Serialize)]
pub struct BasePoints {
    pub team:    i32,
    pub base_id: i32,
    pub points:  i32,

    // #[version([0, 9, 15, 0])]
    pub time_left: Option<i32>,

    // #[version([0, 9, 15, 0])]
    pub invaders_count: Option<i32>,

    pub capturing_stopped: bool,
}

impl BasePoints {
    pub fn parse_from(_gen_info: EntMethodGeneralInfo, arena_data: &[u8]) -> Result<BasePoints, PacketError> {
        let value = make_pickle_val(arena_data)?.list()?;

        if value.len() >= 6 {
            Ok(BasePoints {
                team:              parse_value(0, &value)?,
                base_id:           parse_value(1, &value)?,
                points:            parse_value(2, &value)?,
                time_left:         parse_value(3, &value)?,
                invaders_count:    parse_value(4, &value)?,
                capturing_stopped: parse_truthy_value(5, &value)? != 0,
            })
        } else {
            Ok(BasePoints {
                team:              parse_value(0, &value)?,
                base_id:           parse_value(1, &value)?,
                points:            parse_value(2, &value)?,
                time_left:         None,
                invaders_count:    None,
                capturing_stopped: parse_truthy_value(3, &value)? != 0,
            })
        }
    }
}


// #[derive(Debug, Clone, Serialize, Version)]
// pub struct BaseCaptured {
//     pub team:    i32,
//     pub base_id: i32,
// }

// pub fn parse_base_captured(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
//     let pickle_value = serde_pickle::value_from_slice(
//         arena_data,
//         serde_pickle::DeOptions::new().replace_unresolved_globals(),
//     )?;

//     let PickleVal::Tuple(thing) = pickle_value else {
//         todo!()
//     };

//     Ok(ArenaUpdateData::BaseCaptured(BaseCaptured {
//         team:    parse_value(0, &thing)?,
//         base_id: parse_value(1, &thing)?,
//     }))
// }
