use modular_bitfield::{bitfield, specifiers::{B1, B2, B3}};

use crate::common_structs::constant::LuaConstant;
use crate::common_structs::upvalue::Upvalue;

#[bitfield(filled=false)]
pub struct IsVarargFlag {
    pub has_arg: B1,
    pub is_vararg: B2,
    pub needs_arg: B3
}

pub struct FunctionBlockChunk {
    pub source_name: Option<String>,
    pub source_line_start: u64,
    pub source_line_end: u64,
    pub number_of_parameters: u8,
    pub is_vararg: IsVarargFlag,
    pub maximum_stack_size: u8,
    pub instructions: Vec<u8>,
    pub constants: Vec<LuaConstant>,
    pub upvalues: Vec<Upvalue>
}