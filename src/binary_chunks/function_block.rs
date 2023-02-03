use modular_bitfield::{bitfield, specifiers::{B1, B2, B3}};

#[bitfield(filled=false)]
pub struct IsVarargFlag {
    has_arg: B1,
    is_vararg: B2,
    needs_arg: B3
}

pub struct FunctionBlockChunk {
    pub source_name: Option<String>,
    pub source_line_start: u64,
    pub source_line_end: u64,
    pub number_of_upvalues: u8,
    pub number_of_parameters: u8,
    pub is_vararg: IsVarargFlag,
    pub maximum_stack_size: u8,
}