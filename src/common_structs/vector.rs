use nom::{IResult, multi::length_count};

use super::size_t::lua_size_t;

// Parses a vector
pub fn lua_vector<T, F>(parser: F) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<T>> 
where
    F: Fn(&[u8]) -> IResult<&[u8], T> + Copy
{
    move |input| {
        length_count(lua_size_t, parser)(input)
    }
}