use std::collections::HashMap;

use crate::utils::validate_version;
use crate::wot_data::{EntityType, WotDataForVersion, WOT_DATA_ALL_VERSIONS};
use crate::PacketError;


#[derive(Debug)]
pub struct Context {
    pub(crate) entities:     HashMap<i32, EntityType>,
    pub(crate) players:      HashMap<i32, String>,
    pub(crate) version:      [u16; 4],
    pub(crate) version_data: &'static WotDataForVersion,
}

impl Context {
    pub fn new(version: [u16; 4], players: HashMap<i32, String>) -> Self {
        // let validated_version = validate_version(version);

        Context {
            entities: HashMap::new(),
            players,
            version,
            version_data: WOT_DATA_ALL_VERSIONS
                .get(&utils::version_as_string(validate_version(version)))
                .expect("DEV ERROR: Validated version is invalid"),
        }
    }

    /// This may not be same as the replay version. This version returns a version that is closest to
    /// the actual replay version that we have .def files for
    pub fn get_version(&self) -> [u16; 4] {
        self.version
    }

    pub fn find_entity_type(&self, entity_id: i32) -> Result<EntityType, PacketError> {
        self.entities
            .get(&entity_id)
            .copied()
            .ok_or_else(|| PacketError::NotFoundError {
                err: format!("entity with id: {entity_id} not found for current replay context"),
            })
    }

    pub fn add_entity(&mut self, entity_id: i32, entity_type: EntityType) {
        self.entities.insert(entity_id, entity_type);
    }

    pub fn find_player(&self, id: i32) -> Option<String> {
        self.players.get(&id).map(Into::into)
    }
}
