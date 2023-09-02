use rusty_lua_dec::{lua_file::LuaFile, disassembler::Disassembler};

fn main() {
    let test = include_bytes!(r"..\tests\all_opcodes.luac");
    let file = LuaFile::parse(test).unwrap().1;
    let disasm = Disassembler::new(file);
    println!("{}", disasm.display_header());

}
