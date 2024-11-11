use nom::bytes::complete::take;
use nom::number::complete::*;

use super::types::Vector3;
use super::PacketError;
#[derive(Clone, Copy)]
pub(crate) struct InputStream<'a>(&'a [u8]);


impl<'a> InputStream<'a> {
    pub(crate) fn from(input: &'a [u8]) -> InputStream {
        InputStream(input)
    }

    pub(crate) fn le_i8(&mut self) -> Result<i8, PacketError> {
        let (remaining, val) = le_i8(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_i16(&mut self) -> Result<i16, PacketError> {
        let (remaining, val) = le_i16(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_i32(&mut self) -> Result<i32, PacketError> {
        let (remaining, val) = le_i32(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_i64(&mut self) -> Result<i64, PacketError> {
        let (remaining, val) = le_i64(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_u8(&mut self) -> Result<u8, PacketError> {
        let (remaining, val) = le_u8(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_u16(&mut self) -> Result<u16, PacketError> {
        let (remaining, val) = le_u16(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_u24(&mut self) -> Result<u32, PacketError> {
        let (remaining, val) = le_u24(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_u32(&mut self) -> Result<u32, PacketError> {
        let (remaining, val) = le_u32(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_u64(&mut self) -> Result<u64, PacketError> {
        let (remaining, val) = le_u64(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_f32(&mut self) -> Result<f32, PacketError> {
        let (remaining, val) = le_f32(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn le_f64(&mut self) -> Result<f64, PacketError> {
        let (remaining, val) = le_f64(self.0)?;
        self.0 = remaining;

        Ok(val)
    }

    pub(crate) fn take(&mut self, len: usize) -> Result<&'a [u8], PacketError> {
        let (remaining, took) = take(len)(self.0)?;
        self.0 = remaining;

        Ok(took)
    }

    pub(crate) fn blob(&mut self) -> Result<&'a [u8], PacketError> {
        let mut len = self.le_u8()? as usize;

        if len == u8::MAX as usize {
            // This is a packed int spanning 3 bytes Ex: 0xFF080100
            len = self.le_u24()? as usize;
        }

        let blob = self.take(len)?;

        Ok(blob)
    }

    pub(crate) fn array<T>(
        &mut self, elem_fn: fn(&mut Self) -> Result<T, PacketError>,
    ) -> Result<Vec<T>, PacketError> {
        let len = self.le_u8()? as usize;

        std::iter::repeat_with(|| elem_fn(self)).take(len).collect()
    }

    pub(crate) fn vector3(&mut self) -> Result<Vector3, PacketError> {
        Ok(Vector3 {
            x: self.le_f32()?,
            z: self.le_f32()?,
            y: self.le_f32()?,
        })
    }

    pub(crate) fn remaining_input(self) -> &'a [u8] {
        self.0
    }
}
