use nom::{combinator::map, number::complete::be_i8, sequence::tuple, IResult};

use super::{size_t::lua_size_t, string::lua_string_utf8, vector::lua_vector};

#[derive(Debug, PartialEq)]
pub struct AbsLineInfo {
    pub pc: u64,
    pub line: u64,
}
impl AbsLineInfo {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((lua_size_t, lua_size_t)), |(pc, line)| AbsLineInfo {
            pc,
            line,
        })(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct LocalVar {
    pub name: String,
    pub start_pc: u64,
    pub end_pc: u64,
}
impl LocalVar {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((lua_string_utf8, lua_size_t, lua_size_t)),
            |(name, start_pc, end_pc)| LocalVar {
                name,
                start_pc,
                end_pc,
            },
        )(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct DebugInfo {
    pub line_info: Vec<i8>,
    pub abs_line_info: Vec<AbsLineInfo>,
    pub local_vars: Vec<LocalVar>,
    pub upvalue_names: Vec<String>,
}

impl DebugInfo {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                lua_vector(|input| be_i8(input)),
                lua_vector(AbsLineInfo::parse),
                lua_vector(LocalVar::parse),
                lua_vector(lua_string_utf8),
            )),
            |(line_info, abs_line_info, local_vars, upvalue_names)| DebugInfo {
                line_info,
                abs_line_info,
                local_vars,
                upvalue_names,
            },
        )(input)
    }
}
