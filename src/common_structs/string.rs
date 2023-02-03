use nom::{
    bytes::complete::take,
    combinator::{flat_map, map},
    number::Endianness,
    IResult,
};

use super::size_t::lua_size_t;

/**
 * Parses a lua string
 */
pub fn lua_string(
    endianness: Endianness,
    size_t_size: u8,
) -> impl FnMut(&[u8]) -> IResult<&[u8], Option<&[u8]>> {
    move |input| {
        flat_map(
            lua_size_t(endianness, size_t_size), // First parse the size
            lua_string_data, // parse the string data
        )(input)
    }
}

fn lua_string_data(size: u64) -> impl FnMut(&[u8]) -> IResult<&[u8], Option<&[u8]>> {
    move |input| {
        if size == 0 {  // String does not exist
            return Ok((input, None))  
        }
        map(take(size - 1), |data| Some(data))(input)  // Resolve the string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_does_not_exist() {
        let buf = &[0u8];
        let res = lua_string(Endianness::Little, 1)(buf);
        assert!(res.is_ok());
        assert_eq!(None, res.unwrap().1);
    }
    #[test]
    fn test_empty_string() {
        let buf = &[1u8];
        let res = lua_string(Endianness::Little, 1)(buf);
        assert!(res.is_ok());
        assert_eq!(Some(&b""[..]), res.unwrap().1);
    }
    #[test]
    fn test_regular_string() {
        let buf = &b"\x04abc"[..];
        let res = lua_string(Endianness::Little, 1)(buf);
        assert!(res.is_ok());
        assert_eq!(Some(&b"abc"[..]), res.unwrap().1);
    }
}
