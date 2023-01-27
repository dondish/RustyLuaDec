use std::io;

use crate::struct_io::StructIO;

#[derive(Debug)]
pub struct HeaderChunk {
    pub signature: [u8; 4],
    pub version_number: u8,
    pub format_version: u8,
    pub is_little_endian: bool,
    pub size_of_int: u8,
    pub size_of_size_t: u8,
    pub size_of_instruction: u8,
    pub size_of_lua_number: u8,
    pub is_integral: bool,
}

impl HeaderChunk {
    pub fn parse(struct_io: &mut StructIO) -> io::Result<Self> {
        let mut signature = [0u8; 4];
        struct_io.read_exact(&mut signature)?;
        let version_number = struct_io.read_u8_le()?;
        let format_version = struct_io.read_u8_le()?;
        let is_little_endian = struct_io.read_u8_le()? == 1;
        let size_of_int = struct_io.read_u8_le()?;
        let size_of_size_t = struct_io.read_u8_le()?;
        let size_of_instruction = struct_io.read_u8_le()?;
        let size_of_lua_number = struct_io.read_u8_le()?;
        let is_integral = struct_io.read_u8_le()? == 1;

        Ok(HeaderChunk {
            signature,
            version_number,
            format_version,
            is_little_endian,
            size_of_int,
            size_of_size_t,
            size_of_instruction,
            size_of_lua_number,
            is_integral,
        })
    }
}
