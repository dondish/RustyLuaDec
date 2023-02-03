/*===========================================================================
  We assume that instructions are unsigned 32-bit integers.
  All instructions have an opcode in the first 7 bits.
  Instructions can have the following formats:

        3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0
        1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
iABC          C(8)     |      B(8)     |k|     A(8)      |   Op(7)     |
iABx                Bx(17)               |     A(8)      |   Op(7)     |
iAsBx              sBx (signed)(17)      |     A(8)      |   Op(7)     |
iAx                           Ax(25)                     |   Op(7)     |
isJ                           sJ(25)                     |   Op(7)     |

  A signed argument is represented in excess K: the represented value is
  the written unsigned value minus K, where K is half the maximum for the
  corresponding unsigned argument.
===========================================================================*/

use nom::{combinator::map, number::complete::le_u32, IResult};

// Instruction encoding
pub enum InstructionEncoding {
    IABC {
        c: u8,
        b: u8,
        k: u8,
        a: u8,
        opcode: u8,
    },
    IABx {
        bx: u32,
        a: u8,
        opcode: u8,
    },
    IAsBx {
        sbx: i32,
        a: u8,
        opcode: u8,
    },
    IAx {
        ax: u32,
        opcode: u8,
    },
    IsJ {
        sj: i32,
        opcode: u8,
    },
}

impl InstructionEncoding {
    // Parser for iABC
    pub fn parse_iabc(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instrucion_encoded| InstructionEncoding::IABC {
            c: (instrucion_encoded >> 24) as u8,
            b: (instrucion_encoded >> 16) as u8,
            k: ((instrucion_encoded >> 15) & 0x1) as u8,
            a: (instrucion_encoded >> 7) as u8,
            opcode: (instrucion_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for iABx
    pub fn parse_iabx(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instrucion_encoded| InstructionEncoding::IABx {
            bx: instrucion_encoded >> 15,
            a: (instrucion_encoded >> 7) as u8,
            opcode: (instrucion_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for iAsBx
    pub fn parse_iasbx(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instrucion_encoded| InstructionEncoding::IAsBx {
            sbx: (instrucion_encoded >> 15) as i32 - 0x1ffff,
            a: (instrucion_encoded >> 7) as u8,
            opcode: (instrucion_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for iAx
    pub fn parse_iax(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instrucion_encoded| InstructionEncoding::IAx {
            ax: (instrucion_encoded >> 7),
            opcode: (instrucion_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for isJ
    pub fn parse_isj(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instrucion_encoded| InstructionEncoding::IsJ {
            sj: (instrucion_encoded >> 7) as i32 - 0x1ffffff,
            opcode: (instrucion_encoded & 0x7f) as u8,
        })(input)
    }
}
