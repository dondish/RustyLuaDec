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
        map(le_u32, |instruction_encoded| InstructionEncoding::IABC {
            c: (instruction_encoded >> 24) as u8,
            b: (instruction_encoded >> 16) as u8,
            k: ((instruction_encoded >> 15) & 0x1) as u8,
            a: (instruction_encoded >> 7) as u8,
            opcode: (instruction_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for iABx
    pub fn parse_iabx(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instruction_encoded| InstructionEncoding::IABx {
            bx: instruction_encoded >> 15,
            a: (instruction_encoded >> 7) as u8,
            opcode: (instruction_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for iAsBx
    pub fn parse_iasbx(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instruction_encoded| InstructionEncoding::IAsBx {
            sbx: (instruction_encoded >> 15) as i32 - 0xffff,
            a: (instruction_encoded >> 7) as u8,
            opcode: (instruction_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for iAx
    pub fn parse_iax(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instruction_encoded| InstructionEncoding::IAx {
            ax: (instruction_encoded >> 7),
            opcode: (instruction_encoded & 0x7f) as u8,
        })(input)
    }

    // Parser for isJ
    pub fn parse_isj(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, |instruction_encoded| InstructionEncoding::IsJ {
            sj: (instruction_encoded >> 7) as i32 - 0xffffff,
            opcode: (instruction_encoded & 0x7f) as u8,
        })(input)
    }
}

#[cfg(test)]
mod tests {

    use super::InstructionEncoding;
    use crate::instruction_parsing::opcodes::Opcode;

    #[test]
    fn test_parse_abc() {
        let data: [u8; 0x04] = [0x0F, 0x80, 0x01, 0x02];
        if let (_, InstructionEncoding::IABC { c, b, k, a, opcode }) =
            InstructionEncoding::parse_iabc(&data[..]).unwrap()
        {
            assert_eq!(opcode, Opcode::SetTabup as u8);
            assert_eq!(a, 0);
            assert_eq!(b, 1);
            assert_eq!(c, 2);
            assert_eq!(k, 1);
        } else {
            assert!(
                false,
                "parse_iabc should not return an Ok value with another encoding"
            )
        }
    }

    #[test]
    fn test_parse_abx() {
        let data: [u8; 0x04] = [0x4A, 0x07, 0x00, 0x00];
        if let (_, InstructionEncoding::IABx { bx, a, opcode }) =
            InstructionEncoding::parse_iabx(&data[..]).unwrap()
        {
            assert_eq!(opcode, Opcode::ForPrep as u8);
            assert_eq!(a, 0xe);
            assert_eq!(bx, 0);
        } else {
            assert!(
                false,
                "parse_iabx should not return an Ok value with another encoding"
            )
        }
    }

    #[test]
    fn test_parse_asbx() {
        let data: [u8; 0x04] = [0x01, 0x80, 0xFF, 0x7F];
        if let (_, InstructionEncoding::IAsBx { sbx, a, opcode }) =
            InstructionEncoding::parse_iasbx(&data[..]).unwrap()
        {
            assert_eq!(opcode, Opcode::LoadI as u8);
            assert_eq!(a, 0);
            assert_eq!(sbx, 0);
        } else {
            assert!(
                false,
                "parse_iasbx should not return an Ok value with another encoding"
            )
        }
    }

    #[test]
    fn test_parse_ax() {
        let data: [u8; 0x04] = [0x52, 0x00, 0x00, 0x00];
        if let (_, InstructionEncoding::IAx { ax, opcode }) =
            InstructionEncoding::parse_iax(&data[..]).unwrap()
        {
            assert_eq!(opcode, Opcode::Extraarg as u8);
            assert_eq!(ax, 0);
        } else {
            assert!(
                false,
                "parse_iax should not return an Ok value with another encoding"
            )
        }
    }

    #[test]
    fn test_parse_sj() {
        let data: [u8; 0x04] = [0x38, 0x00, 0x00, 0x80];
        if let (_, InstructionEncoding::IsJ { sj, opcode }) =
            InstructionEncoding::parse_isj(&data[..]).unwrap()
        {
            assert_eq!(opcode, Opcode::Jmp as u8);
            assert_eq!(sj, 1);
        } else {
            assert!(
                false,
                "parse_isj should not return an Ok value with another encoding"
            )
        }
    }
}
