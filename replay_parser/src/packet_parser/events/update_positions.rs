use std::collections::HashMap;

use super::EntMethodGeneralInfo;
use crate::packet_parser::prelude::*;
use crate::packet_parser::InputStream;

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePositions(HashMap<i32, Vec<i16>>);


impl UpdatePositions {
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, data: &[u8], context: &Context,
    ) -> Result<EventType, PacketError> {
        let mut reader = InputStream::from(data);

        let res = if context.version < [1, 9, 0, 0] {
            let indices = reader.array(|it| InputStream::le_u8(it).map(u16::from))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else if context.version >= [1, 9, 0, 0] {
            let indices = reader.array(|it| InputStream::le_u16(it))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else if context.version >= [1, 10, 0, 0] {
            let indices = reader.array(|it| InputStream::le_u8(it).map(u16::from))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else if context.version >= [1, 10, 1, 0] {
            let indices = reader.array(|it| InputStream::le_u16(it))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else if context.version >= [1, 11, 0, 0] {
            let indices = reader.array(|it| InputStream::le_u8(it).map(u16::from))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else if context.version >= [1, 14, 1, 0] {
            let indices = reader.array(|it| InputStream::le_u16(it))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else if context.version >= [1, 15, 0, 0] {
            let indices = reader.array(|it| InputStream::le_u8(it).map(u16::from))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else if context.version >= [1, 26, 1, 0] {
            let indices = reader.array(|it| InputStream::le_u16(it))?;
            let positions = reader.array(InputStream::le_i16)?;

            UpdatePositions(make_positions_map(indices, positions, context))
        } else {
            unreachable!()
        };

        Ok(EventType::UpdatePositions(res))
    }
}

fn make_positions_map(indices: Vec<u16>, positions: Vec<i16>, context: &Context) -> HashMap<i32, Vec<i16>> {
    let keys = indices
        .into_iter()
        .flat_map(|idx| context.players_index.get(&(idx as usize)))
        .copied();

    keys.zip(positions.chunks(2).map(|pos| pos.to_vec())).collect()
}
