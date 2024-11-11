/// `entity_method` describe multiple events because there can be many different types of method calls
pub(crate) mod entity_method;
// pub use entity_method::avatar_methods::update_arena::ArenaUpdateData;
// pub use entity_method::vehicle_methods::*;
// pub use entity_method::EntityMethod;
pub use entity_method::EntMethodGeneralInfo;
mod game_version;
pub use game_version::GameVersion;

mod avatar_create;
pub use avatar_create::CreateAvatar;

mod position;
pub use position::Position;

mod chat;
pub use chat::Chat;

mod entity_create;
pub use entity_create::CreateEntity;

mod crypto_key;
pub use crypto_key::CryptoKey;

// mod entity_property;
// pub use entity_property::EntityProperty;
// pub use entity_property::EntityPropertyEvent;

mod avatar_ready;
pub use avatar_ready::AvatarReady;

mod vehicle_added;
pub use vehicle_added::VehicleAdded;

mod vehicle_list;
pub use vehicle_list::VehicleList;

mod vehicle_descr;
pub use vehicle_descr::VehicleDescr;

mod vehicle_killed;
pub use vehicle_killed::VehicleKilled;

mod vehicle_updated;
pub use vehicle_updated::VehicleUpdated;

mod vehicle_statistics;
pub use vehicle_statistics::VehicleStatistics;

mod statistics;
pub use statistics::Statistics;

mod base_points;
pub use base_points::BasePoints;

mod base_captured;
pub use base_captured::BaseCaptured;

mod period;
pub use period::Period;

mod fog_of_war;
pub use fog_of_war::FogOfWar;

mod view_points;
pub use view_points::ViewPoints;


mod team_killer;
pub use team_killer::TeamKiller;

mod update_vehicle_ammo;
pub use update_vehicle_ammo::UpdateVehicleAmmo;

mod update_positions;
pub use update_positions::UpdatePositions;
