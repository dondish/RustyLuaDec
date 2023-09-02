use nom::{IResult, sequence::tuple, combinator::map, number::complete::be_u8};

use crate::binary_chunks::{header::HeaderChunk, function_block::FunctionBlockChunk};

/// Compiled Lua File
#[derive(Debug, PartialEq)]
pub struct LuaFile {
    pub header: HeaderChunk,
    pub number_of_upvalues: u8,
    pub main_function_block: FunctionBlockChunk
}

impl LuaFile {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple(
            (
                HeaderChunk::parse,
                be_u8,
                FunctionBlockChunk::parse
            )
        ), |(header, number_of_upvalues, main_function_block)| {
            LuaFile {
                header, 
                number_of_upvalues, 
                main_function_block
            }
        })(input)
    }
}