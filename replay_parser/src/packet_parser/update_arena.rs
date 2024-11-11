use wot_types::ArenaUpdate;

use super::events::{
    BaseCaptured, BasePoints, EntMethodGeneralInfo, FogOfWar, Period, Statistics, TeamKiller, VehicleAdded,
    VehicleDescr, VehicleKilled, VehicleList, VehicleStatistics, VehicleUpdated, ViewPoints,
};
use super::{Context, EventType, InputStream, PacketError};
use crate::events::AvatarReady;


pub(crate) fn parse_update_arena_method(
    gen_info: EntMethodGeneralInfo, payload: &[u8], context: &Context,
) -> Result<EventType, PacketError> {
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
        ArenaUpdate::AvatarReady => EventType::AvatarReady(AvatarReady::parse_from(gen_info, arena_data)?),
        ArenaUpdate::VehicleAdded => EventType::VehicleAdded(VehicleAdded::parse_from(gen_info, arena_data)?),
        ArenaUpdate::VehicleList => EventType::VehicleList(VehicleList::parse_from(gen_info, arena_data)?),
        ArenaUpdate::VehicleDescr => EventType::VehicleDescr(VehicleDescr::parse_from(gen_info, arena_data)?),
        ArenaUpdate::BasePoints => EventType::BasePoints(BasePoints::parse_from(gen_info, arena_data)?),
        ArenaUpdate::BaseCaptured => EventType::BaseCaptured(BaseCaptured::parse_from(gen_info, arena_data)?),
        ArenaUpdate::VehicleStatistics => {
            EventType::VehicleStatistics(VehicleStatistics::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::VehicleUpdated => {
            EventType::VehicleUpdated(VehicleUpdated::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::VehicleKilled => {
            EventType::VehicleKilled(VehicleKilled::parse_from(gen_info, arena_data)?)
        }
        ArenaUpdate::Period => EventType::Period(Period::parse_from(gen_info, arena_data)?),
        ArenaUpdate::Statistics => EventType::Statistics(Statistics::parse_from(gen_info, arena_data)?),
        ArenaUpdate::FogOfWar => EventType::FogOfWar(FogOfWar::parse_from(gen_info, arena_data)?),
        ArenaUpdate::ViewPoints => EventType::ViewPoints(ViewPoints::parse_from(gen_info, arena_data)?),
        ArenaUpdate::TeamKiller => {
            EventType::TeamKiller(TeamKiller::parse_from(gen_info, arena_data, context)?)
        }
        _ => EventType::UnimplementedArenaUpdate(update_type.to_string()),
    };

    Ok(result)
}
