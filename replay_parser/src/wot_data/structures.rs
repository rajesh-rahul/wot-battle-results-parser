use miniz_oxide::inflate::decompress_to_vec_zlib;
use wot_types::WotValue;

use super::EntityType;
use crate::packet_parser::InputStream;
use crate::{PacketError, PacketName};

#[derive(Debug)]
pub(crate) struct WotDataForVersion {
    pub(crate) packet_map:      phf::Map<u32, PacketName>,
    pub(crate) entities:        &'static [EntityData],
    pub(crate) special_formats: SpecialFormat,
}

impl WotDataForVersion {
    pub(crate) fn packet_name(&self, packet_ty: u32) -> PacketName {
        self.packet_map
            .get(&packet_ty)
            .copied()
            .unwrap_or(PacketName::Unimplemented)
    }
}

#[derive(Debug)]
pub(crate) struct EntityData {
    pub(crate) ty:              EntityType,
    pub(crate) exposed_props:   &'static [PropData],
    pub(crate) exposed_methods: &'static [MethodData],
}

#[derive(Debug)]
pub(crate) struct SpecialFormat {
    pub(crate) create_avatar: &'static [PropData],
}

#[derive(Debug)]
pub(crate) struct PropData {
    pub(crate) name:          &'static str,
    pub(crate) default_value: WotXmlDefaultValue,
    pub(crate) datatype:      WotXmlType,
    pub(crate) size:          u32,
}

#[derive(Debug)]
pub(crate) struct MethodData {
    pub(crate) name: &'static str,
    pub(crate) args: &'static [WotXmlType],
    pub(crate) size: usize,
}

#[derive(Debug)]
pub(crate) struct MethodArgData {
    pub(crate) name:     &'static str,
    pub(crate) datatype: WotXmlType,
}

#[derive(Debug)]
pub(crate) struct DataTypeData {
    pub(crate) name: &'static str,
}

#[derive(Debug)]
pub(crate) enum WotXmlDefaultValue {
    Float(f32),
    Str(&'static str),
    Bool(bool),
    Int(i32),
    List(&'static [WotXmlDefaultValue]),
    Json(&'static str),
    EmptyDict,
    Null,
}

impl Into<serde_json::Value> for WotXmlDefaultValue {
    fn into(self) -> serde_json::Value {
        todo!()
    }
}


#[derive(Debug)]
pub(crate) enum WotXmlType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Blob,
    String,
    Python,
    Vector2,
    Vector3,
    Vector4,
    // MailBox,
    Array {
        inner: &'static WotXmlType,
        size:  Option<usize>,
    },
    FixedDict {
        inner:      &'static [(&'static str, WotXmlType)],
        allow_none: bool,
    },
    // UserType,
    RecursivePython,
}


impl WotXmlType {
    pub fn input_to_json_value<'a>(
        &self, input: &'a [u8],
    ) -> Result<(&'a [u8], serde_json::Value), PacketError> {
        use nom::bytes::complete::take;
        use nom::number::complete::*;

        match self {
            WotXmlType::Int8 => {
                let (remaining, result) = le_i8(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::Int16 => {
                let (remaining, result) = le_i16(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::Int32 => {
                let (remaining, result) = le_i32(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::Int64 => {
                let (remaining, result) = le_i64(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::UInt8 => {
                let (remaining, result) = le_u8(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::UInt16 => {
                let (remaining, result) = le_u16(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::UInt32 => {
                let (remaining, result) = le_u32(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::UInt64 => {
                let (remaining, result) = le_u64(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::Float32 => {
                let (remaining, result) = le_f32(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::Float64 => {
                let (remaining, result) = le_f64(input)?;

                Ok((remaining, result.into()))
            }
            WotXmlType::Blob => {
                let (remaining, len) = le_u8(input)?;

                if len == u8::MAX {
                    // This is a packed int spanning 3 bytes Ex: 0xFF080100
                    let (remaining, len) = le_u24(remaining)?;
                    let (remaining, bytes_array) = take(len)(remaining)?;

                    Ok((remaining, hex::encode(bytes_array).into()))
                } else {
                    let (remaining, bytes_array) = take(len)(remaining)?;

                    Ok((remaining, hex::encode(bytes_array).into()))
                }
            }
            WotXmlType::String => {
                let (mut remaining, len) = le_u8(input)?;

                let actual_len = if len == u8::MAX {
                    // This is a packed int spanning 3 bytes Ex: 0xFF080100
                    let (r, len) = le_u24(remaining)?;
                    remaining = r;

                    len
                } else {
                    len as u32
                };

                let (remaining, str_bytes) = take(actual_len)(remaining)?;

                match std::str::from_utf8(str_bytes) {
                    Ok(result) => Ok((remaining, result.into())),
                    Err(_) => Ok((remaining, hex::encode(str_bytes).into())),
                }
            }
            WotXmlType::Python => {
                let (mut remaining, len) = le_u8(input)?;

                let actual_len = if len == u8::MAX {
                    // This is a packed int spanning 3 bytes Ex: 0xFF080100
                    let (r, len) = le_u24(remaining)?;
                    remaining = r;

                    len
                } else {
                    len as u32
                };

                let (remaining, bytes_array) = take(actual_len)(remaining)?;

                let pickle_data = if matches!(bytes_array, [0x78, 0x9C, ..]) {
                    &decompress_to_vec_zlib(input).map_err(|_| PacketError::ConversionError {
                        err: "decompression error".to_string(),
                    })?
                } else {
                    bytes_array
                };

                let pickle_value = match serde_pickle::value_from_slice(
                    pickle_data,
                    serde_pickle::DeOptions::new().replace_unresolved_globals(),
                ) {
                    Ok(val) => val,
                    Err(_) => {
                        return Ok((
                            remaining,
                            serde_json::Value::String("Unsupported Python".to_string()),
                        ))
                    }
                };

                let wot_value: WotValue = serde_pickle::from_value(pickle_value)?;
                Ok((
                    remaining,
                    serde_json::to_value(wot_value)
                        .map_err(|err| PacketError::ConversionError { err: err.to_string() })?,
                ))
            }
            WotXmlType::Vector2 => {
                let (remaining, a) = le_f32(input)?;
                let (remaining, b) = le_f32(remaining)?;

                Ok((remaining, vec![a, b].into()))
            }
            WotXmlType::Vector3 => {
                let (remaining, a) = le_f32(input)?;
                let (remaining, b) = le_f32(remaining)?;
                let (remaining, c) = le_f32(remaining)?;

                Ok((remaining, vec![a, b, c].into()))
            }
            WotXmlType::Vector4 => {
                let (remaining, a) = le_f32(input)?;
                let (remaining, b) = le_f32(remaining)?;
                let (remaining, c) = le_f32(remaining)?;
                let (remaining, d) = le_f32(remaining)?;

                Ok((remaining, vec![a, b, c, d].into()))
            }
            WotXmlType::Array { inner, size } => {
                let mut arr = Vec::new();
                let mut remaining = input;

                let arr_size = if size.is_some() {
                    size.unwrap()
                } else {
                    let (arr_remaining, size) = le_u8(remaining)?;
                    remaining = arr_remaining;

                    size as usize
                };


                for _ in 0..arr_size {
                    let (input, item) = inner.input_to_json_value(remaining)?;
                    remaining = input;
                    arr.push(item);
                }

                Ok((remaining, serde_json::Value::Array(arr)))
            }
            WotXmlType::FixedDict { inner, allow_none } => {
                let mut map = serde_json::Map::with_capacity(inner.len());

                let mut reader = InputStream::from(input);
                // CREDIT: https://github.com/Monstrofil/replays_unpack
                if *allow_none {
                    let is_none = reader.le_u8()? == 0;

                    if is_none {
                        return Ok((reader.remaining_input(), serde_json::Value::Null));
                    }
                }

                let mut remaining = reader.remaining_input();
                for (k, v) in *inner {
                    let (r, value_parsed) = v.input_to_json_value(remaining)?;
                    remaining = r;

                    map.insert(k.to_string(), value_parsed);
                }

                Ok((remaining, serde_json::Value::Object(map)))
            }
            WotXmlType::RecursivePython => unreachable!(),
        }
    }
}
