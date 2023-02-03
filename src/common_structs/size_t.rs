use nom::IResult;

/**
 * Factory for parsing a size_t
 */
pub fn lua_size_t(input: &[u8]) -> IResult<&[u8], u64> {
    let mut size = 0;
    let mut current_size = 0u64;
    loop {
        if input.len() == size {
            return Err(nom::Err::Incomplete(nom::Needed::new(size+1)));
        }
        current_size <<= 7;
        current_size |= (input[size] & 0x7f) as u64;
        
        if (input[size] & 0x80) != 0 {
            break
        }
        size+=1;
    }
    Ok((&input[(size+1)..], current_size))
}