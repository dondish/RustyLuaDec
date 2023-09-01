use nom::{
    bytes::complete::tag,
    combinator::map,
    number::complete::{be_u8, le_f64, le_u64},
    sequence::tuple,
    IResult,
};

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
            minor: value & 0xF,
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
    pub size_of_int: u8,
    pub size_of_size_t: u8,
    pub size_of_lua_number: u8,
}

impl HeaderChunk {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                tag("\x1BLua"),
                be_u8,
                be_u8,
                tag(b"\x19\x93\x0d\x0a\x1a\x0a"),
                be_u8,
                be_u8,
                be_u8,
                le_u64,
                le_f64,
            )),
            |(
                _,
                version_number,
                format_version,
                _,
                size_of_int,
                size_of_size_t,
                size_of_lua_number,
                _,
                _,
            )| {
                HeaderChunk {
                    version_number: version_number.into(),
                    format_version,
                    size_of_int,
                    size_of_size_t,
                    size_of_lua_number,
                }
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_parsing_of_lua_header_chunk() {
        let test_data =
            hex::decode("1B4C7561540019930D0A1A0A0408087856000000000000000000000000287740")
                .unwrap();
        let header_chunk: HeaderChunk = HeaderChunk::parse(&test_data).unwrap().1;
        assert_eq!(
            HeaderChunk {
                version_number: HeaderVersion { major: 5, minor: 4 },
                format_version: 0,
                size_of_int: 4,
                size_of_size_t: 8,
                size_of_lua_number: 8,
            },
            header_chunk
        );
    }
}
