use nom::{
    combinator::map,
    error::ErrorKind,
    number::complete::{le_f64, le_i64, le_u8},
    IResult,
};

use super::string::lua_string;

/// A constant in the function block binary chunk
pub enum LuaConstant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    String(Option<String>),
}

impl LuaConstant {
    pub fn parse(input: &[u8]) -> IResult<&[u8], LuaConstant> {
        let (input, type_) = le_u8(input)?;

        match type_ {
            0x0 => Ok((input, LuaConstant::Nil)),
            0x1 => Ok((input, LuaConstant::Boolean(false))),
            0x3 => map(le_i64, LuaConstant::Integer)(input),
            0x4 | 0x14 => map(lua_string, |str_bytes_opt| {
                LuaConstant::String(
                    str_bytes_opt.map(|input| String::from_utf8_lossy(input).to_string()),
                )
            })(input),
            0x11 => Ok((input, LuaConstant::Boolean(true))),
            0x13 => map(le_f64, LuaConstant::Number)(input),
            _ => Err(nom::Err::Failure(nom::error::Error {
                input,
                code: ErrorKind::Fail,
            })),
        }
    }
}
