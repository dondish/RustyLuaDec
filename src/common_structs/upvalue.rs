use nom::{
    combinator::map_res,
    number::complete::be_u8,
    sequence::tuple,
    IResult,
};

use num::FromPrimitive;

use super::variable_kind::VariableKind;

#[derive(Debug, PartialEq)]
pub struct Upvalue {
    pub in_stack: bool,
    pub index: u8,
    pub kind: VariableKind,
}

impl Upvalue {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(
            tuple((be_u8, be_u8, be_u8)),
            |(in_stack_byte, index, kind_byte)| {
                if let Some(variable_kind) = VariableKind::from_u8(kind_byte) {
                Ok(Upvalue {
                    in_stack: in_stack_byte == 1,
                    index: index,
                    kind: variable_kind,
                })

                } else {
                    Err(nom::Err::Failure("Invalid variable kind"))
                }
            },
        )(input)
    }
}
