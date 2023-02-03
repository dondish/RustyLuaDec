use nom::{IResult, number::{Endianness, complete}};

/**
 * Macro to convert an IResult<_, uX> where X is not 64 to an IResult<_, u64>
 */
macro_rules! to_u64_ires {
    ($exp:expr) => {
        $exp.map(|(input, data)| (input, data as u64))
    };
}

/**
 * Factory for parsing a size_t
 */
pub fn lua_size_t(endianness: Endianness, size_t_size: u8) -> impl FnMut(&[u8]) -> IResult<&[u8], u64> {
    move |input| match size_t_size {
        1 => to_u64_ires!(complete::u8(input)),
        2 => to_u64_ires!(complete::u16(endianness)(input)),
        4 => to_u64_ires!(complete::u32(endianness)(input)),
        8 => complete::u64(endianness)(input),
        _ => panic!("Invalid size of size_t used")
    }
}