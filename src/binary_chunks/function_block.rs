use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B3},
};
use nom::{
    combinator::map,
    number::complete::{be_u8, le_u32},
    sequence::tuple,
    IResult,
};

use crate::common_structs::{
    constant::LuaConstant, debug_info::DebugInfo, string::lua_string_utf8, vector::lua_vector,
};
use crate::common_structs::{size_t::lua_size_t, upvalue::Upvalue};

#[bitfield(filled = false)]
#[derive(Debug, PartialEq)]
pub struct IsVarargFlag {
    pub has_arg: B1,
    pub is_vararg: B2,
    pub needs_arg: B3,
}

#[derive(Debug, PartialEq)]
pub struct FunctionBlockChunk {
    pub source_name: Option<String>,
    pub source_line_start: u64,
    pub source_line_end: u64,
    pub number_of_parameters: u8,
    pub is_vararg: IsVarargFlag,
    pub maximum_stack_size: u8,
    pub instructions: Vec<u32>,
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
                lua_vector(|input| le_u32(input)),
                lua_vector(LuaConstant::parse),
                lua_vector(Upvalue::parse),
                lua_vector(Self::parse),
                DebugInfo::parse,
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
                debug_info,
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
                    debug_info,
                }
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        binary_chunks::function_block::IsVarargFlag,
        common_structs::{debug_info::DebugInfo, upvalue::Upvalue, variable_kind::VariableKind},
    };

    use super::FunctionBlockChunk;

    #[test]
    fn test_function_block_chunk_parsing() {
        let data: [u8; 0x22] = [
            0x80, 0xCD, 0xCF, 0x00, 0x00, 0x02, 0x83, 0x09, 0x00, 0x00, 0x00, 0x48, 0x00, 0x02,
            0x00, 0x47, 0x00, 0x01, 0x00, 0x80, 0x81, 0x01, 0x06, 0x00, 0x80, 0x83, 0x01, 0x00,
            0x01, 0x80, 0x80, 0x81, 0x82, 0x78,
        ];
        let function_block_chunk_res = FunctionBlockChunk::parse(&data);
        assert!(function_block_chunk_res.is_ok());
        assert_eq!(
            function_block_chunk_res.unwrap().1,
            FunctionBlockChunk {
                source_name: None,
                source_line_start: 77,
                source_line_end: 79,
                number_of_parameters: 0,
                is_vararg: IsVarargFlag::new(),
                maximum_stack_size: 2,
                instructions: vec![9, 131144, 65607],
                constants: vec![],
                upvalues: vec![Upvalue {
                    in_stack: true,
                    index: 6,
                    kind: VariableKind::Regular
                }],
                protos: vec![],
                debug_info: DebugInfo {
                    line_info: vec![1, 0, 1],
                    abs_line_info: vec![],
                    local_vars: vec![],
                    upvalue_names: vec![Some("x".to_string())]
                }
            }
        );
    }
}
