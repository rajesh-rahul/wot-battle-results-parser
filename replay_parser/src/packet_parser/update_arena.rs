use wot_types::ArenaUpdate;

use super::events::{
    BaseCaptured, BasePoints, EntMethodGeneralInfo, FogOfWar, Period, Statistics, VehicleAdded, VehicleDescr,
    VehicleKilled, VehicleList, VehicleStatistics, VehicleUpdated,
};
use super::{BattleEvent, Context, InputStream, PacketError};
use crate::events::AvatarReady;


pub(crate) fn parse_update_arena_method(
    gen_info: EntMethodGeneralInfo, payload: &[u8], _context: &Context,
) -> Result<BattleEvent, PacketError> {
    let mut reader = InputStream::from(payload);

    let update_type = reader.le_u8()?;
    let Ok(update_type) = ArenaUpdate::try_from(update_type as i32) else {
        return Err(PacketError::Misc {
            err: format!("{update_type} does not map to an ArenaUpdate"),
        });
    };

    let arena_data = reader.blob()?;

    if !reader.remaining_input().is_empty() {
        return Err(PacketError::Misc {
            err: format!("input lef over while processing arena update: {update_type}"),
        });
    }


    let result = match update_type {
        ArenaUpdate::AvatarReady => BattleEvent::AvatarReady(AvatarReady::parse_from(gen_info, arena_data)?),
        ArenaUpdate::VehicleAdded => {
            BattleEvent::VehicleAdded(VehicleAdded::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::VehicleList => BattleEvent::VehicleList(VehicleList::parse_from(gen_info, arena_data)?),
        ArenaUpdate::VehicleDescr => {
            BattleEvent::VehicleDescr(VehicleDescr::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::BasePoints => BattleEvent::BasePoints(BasePoints::parse_from(gen_info, arena_data)?),
        ArenaUpdate::BaseCaptured => {
            BattleEvent::BaseCaptured(BaseCaptured::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::VehicleStatistics => {
            BattleEvent::VehicleStatistics(VehicleStatistics::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::VehicleUpdated => {
            BattleEvent::VehicleUpdated(VehicleUpdated::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::VehicleKilled => {
            BattleEvent::VehicleKilled(VehicleKilled::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::Period => BattleEvent::Period(Period::parse_from(gen_info, arena_data)?),
        ArenaUpdate::Statistics => BattleEvent::Statistics(Statistics::parse_from(gen_info, arena_data)?),
        ArenaUpdate::FogOfWar => BattleEvent::FogOfWar(FogOfWar::parse_from(gen_info, arena_data)?),
        _ => BattleEvent::UnimplementedArenaUpdate(update_type.to_string()),
    };

    Ok(result)
}
