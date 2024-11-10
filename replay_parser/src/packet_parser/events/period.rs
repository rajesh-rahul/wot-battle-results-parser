use wot_types::{ArenaPeriod, FinishReason, WotValue};

use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::utils::{decompress_vec, make_pickle_val, parse_value, PickleOps};

#[derive(Debug, Clone, Serialize, Version)]
pub struct Period {
    pub period:          ArenaPeriod,
    pub end_time:        f32,
    pub length:          f32,
    pub additional_info: WotValue,
}
impl Period {
    pub fn parse_from(_gen_info: EntMethodGeneralInfo, arena_data: &[u8]) -> Result<Period, PacketError> {
        let mut value = make_pickle_val(&decompress_vec(arena_data)?)?.list()?;

        let period_type: i32 = parse_value(0, &value)?;

        // let additional_info = match &thing[3] {
        //     PickleVal::Tuple(info) => {
        //         let finish_reason = parse_value::<i32>(1, &info)?;
        //         Ok(PeriodAdditionalInfo::ArenaEnded(ArenaEnded {
        //             winner_team:   parse_value(0, &info)?,
        //             finish_reason: FinishReason::try_from(finish_reason).map_err(|_| {
        //                 PacketError::WrongEnumVariant {
        //                     err: format!("finish reason of {finish_reason} is invalid"),
        //                 }
        //             })?,
        //         }))
        //     }
        //     PickleVal::List(start_times) => {
        //         let mut res: Vec<f32> = vec![];
        //         for i in 0..start_times.len() {
        //             res.push(parse_value::<f32>(i, start_times)?);
        //         }
        //         Ok(PeriodAdditionalInfo::ActivitiesStartTimes(res))
        //     }
        //     _ => Err(PacketError::PickleError {
        //         err: format!("Invalid additional info payload"),
        //     }),
        // }?;

        let additional_info = value
            .get_mut(3)
            .map(|it| serde_pickle::from_value(std::mem::replace(it, serde_pickle::Value::None)))
            .unwrap_or(Ok(WotValue::None))?;

        let period = Period {
            period: ArenaPeriod::try_from(period_type as i32).map_err(|_| PacketError::Misc {
                err: format!("arena period of {period_type} is invalid"),
            })?,
            end_time: parse_value(1, &value)?,
            length: parse_value(2, &value)?,
            additional_info,
        };

        Ok(period)
    }
}

#[derive(Debug, Clone, Serialize, Version)]
pub struct ArenaEnded {
    pub winner_team:   i32,
    pub finish_reason: FinishReason,
}

#[derive(Debug, Clone, Serialize)]
pub enum PeriodAdditionalInfo {
    ActivitiesStartTimes(Vec<f32>),
    ArenaEnded(ArenaEnded),
}
