use crate::lua_file::LuaFile;

pub struct Disassembler {
    lua_file: LuaFile,
}
const UNKNOWN_FILE_NAME: &'static str = "Unknown";

impl Disassembler {
    pub fn new(file: LuaFile) -> Self {
        Disassembler { lua_file: file }
    }

    pub fn display_header(&self) -> String {
        let unknown_file_name = UNKNOWN_FILE_NAME.to_string();

        let file_name = self
            .lua_file
            .main_function_block
            .source_name
            .as_ref()
            .unwrap_or(&unknown_file_name);
        format!(
            indoc!(
                "Lua Compiled File
                Source File Name: {}
                Lua Version: {}
                Format Version: {}
                sizeof(Instruction): {}
                sizeof(lua_Number): {}
                sizeof(lua_Integer): {}"
            ),
            file_name,
            self.lua_file.header.version_number,
            self.lua_file.header.format_version,
            self.lua_file.header.size_of_int,
            self.lua_file.header.size_of_lua_number,
            self.lua_file.header.size_of_size_t
        )
    }
}
