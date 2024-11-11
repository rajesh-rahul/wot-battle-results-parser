use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Serialize, Debug, Clone)]
pub struct ShowShooting {
    pub vehicle_id: i32,

    /// This value seems to be `1` most times. Perhaps, with a double-barrel tank it could different.
    pub burst_count:  u8,
    pub current_guns: Option<u8>, // used to be called gun index
}


impl ShowShooting {
    pub fn parse_from(
        gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        // For this event, `current_guns` field comes and goes so we will use this approach instead
        // of enumerating each different version

        let burst_count = reader.le_u8()?;
        let current_guns = if reader.is_empty() {
            None
        } else {
            Some(reader.le_u8()?)
        };

        Ok(EventType::ShowShooting(ShowShooting {
            vehicle_id: gen_info.entity_id,
            burst_count,
            current_guns,
        }))
    }
}
