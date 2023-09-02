use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B3},
};
use nom::{
    combinator::map,
    number::complete::be_u8,
    sequence::tuple,
    IResult,
};

use crate::common_structs::{upvalue::Upvalue, size_t::lua_size_t};
use crate::common_structs::{
    constant::LuaConstant, debug_info::DebugInfo, string::lua_string_utf8, vector::lua_vector,
};

#[bitfield(filled = false)]
pub struct IsVarargFlag {
    pub has_arg: B1,
    pub is_vararg: B2,
    pub needs_arg: B3,
}

pub struct FunctionBlockChunk {
    pub source_name: String,
    pub source_line_start: u64,
    pub source_line_end: u64,
    pub number_of_parameters: u8,
    pub is_vararg: IsVarargFlag,
    pub maximum_stack_size: u8,
    pub instructions: Vec<u8>,
    pub constants: Vec<LuaConstant>,
    pub upvalues: Vec<Upvalue>,
    pub protos: Vec<FunctionBlockChunk>,
    pub debug_info: DebugInfo,
}

impl FunctionBlockChunk {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                lua_string_utf8,
                lua_size_t,
                lua_size_t,
                be_u8,
                be_u8,
                be_u8,
                lua_vector(|input| be_u8(input)),
                lua_vector(
                    LuaConstant::parse
                ),
                lua_vector(Upvalue::parse),
                lua_vector(Self::parse),
                DebugInfo::parse
            )),
            |(
                source_name,
                source_line_start,
                source_line_end,
                number_of_parameters,
                is_vararg,
                maximum_stack_size,
                instructions,
                constants,
                upvalues,
                protos,
                debug_info
            )| {
                FunctionBlockChunk {
                    source_name,
                    source_line_start,
                    source_line_end,
                    number_of_parameters,
                    is_vararg: IsVarargFlag::from_bytes([is_vararg; 1]).unwrap(),
                    maximum_stack_size,
                    instructions,
                    constants,
                    upvalues,
                    protos,
                    debug_info
                }
            },
        )(input)
    }
}
