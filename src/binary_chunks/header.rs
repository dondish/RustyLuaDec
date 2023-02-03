use nom::{IResult, combinator::map, sequence::tuple, bytes::complete::tag, number::complete::be_u8};


/**
 * Version of a Lua 5 header block
 */
#[derive(Debug, PartialEq, Eq)]
pub struct HeaderVersion {
    major: u8,
    minor: u8,
}

impl From<u8> for HeaderVersion {
    fn from(value: u8) -> Self {
        Self {
            major: value >> 4,
            minor: value & 0xF
        }
    }
}

/**
 * Header block of a Lua 5 binary chunk
 */
#[derive(Debug, PartialEq, Eq)]
pub struct HeaderChunk {
    pub version_number: HeaderVersion,
    pub format_version: u8,
    pub is_little_endian: bool,
    pub size_of_int: u8,
    pub size_of_size_t: u8,
    pub size_of_instruction: u8,
    pub size_of_lua_number: u8,
    pub is_integral: bool,
}

impl HeaderChunk {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((tag("\x1BLua"), be_u8, be_u8, be_u8, be_u8, be_u8, be_u8, be_u8, be_u8)), 
        |(_, version_number, format_version, is_little_endian, size_of_int, size_of_size_t, size_of_instruction, size_of_lua_number, is_integral)| {
            HeaderChunk {
                version_number: version_number.into(),
                format_version,
                is_little_endian:  is_little_endian == 1,
                size_of_int,
                size_of_size_t,
                size_of_instruction,
                size_of_lua_number,
                is_integral: is_integral == 1
            }
        }
        )(input)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_parsing_of_lua_5_1_header_chunk() {
        let test_data = hex::decode("1B4C75615100010404040800").unwrap();
        let header_chunk: HeaderChunk = HeaderChunk::parse(&test_data).unwrap().1;
        assert_eq!(
            HeaderChunk {
                version_number: HeaderVersion { major: 5, minor: 1 },
                format_version: 0,
                is_little_endian: true,
                size_of_int: 4,
                size_of_size_t: 4,
                size_of_instruction: 4,
                size_of_lua_number: 8,
                is_integral: false
            },
            header_chunk
        );
    }
}
