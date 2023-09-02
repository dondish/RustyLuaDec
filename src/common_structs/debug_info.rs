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

#[cfg(test)]
mod tests {
    use super::{AbsLineInfo, DebugInfo, LocalVar};

    #[test]
    fn test_abs_line_info_parsing() {
        let data: [u8; 0x03] = [0x01, 0x80, 0xAD];
        let abs_line_info_res = AbsLineInfo::parse(&data);
        assert!(abs_line_info_res.is_ok());
        assert_eq!(
            abs_line_info_res.unwrap().1,
            AbsLineInfo { pc: 128, line: 45 }
        )
    }

    #[test]
    fn test_local_var_parsing() {
        let data: [u8; 0x05] = [0x82, 0x72, 0x82, 0x01, 0xC0];
        let local_var_res = LocalVar::parse(&data);
        assert!(local_var_res.is_ok());
        assert_eq!(
            local_var_res.unwrap().1,
            LocalVar {
                name: "r".to_string(),
                start_pc: 2,
                end_pc: 192
            }
        )
    }

    #[test]
    fn test_debug_info_parsing() {
        let data: [u8; 0x0D] = [
            0x87, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x80, 0x81, 0x82, 0x78,
        ];
        let debug_info_res = DebugInfo::parse(&data);
        assert!(debug_info_res.is_ok());
        assert_eq!(
            debug_info_res.unwrap().1,
            DebugInfo {
                line_info: vec![1, 0, 0, 0, 0, 0, 1],
                abs_line_info: vec![],
                local_vars: vec![],
                upvalue_names: vec!["x".to_string()],
            }
        )
    }
}
