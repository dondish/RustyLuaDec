use std::io::{self, Read};
use std::mem::size_of;

// Implements IO methods for efficiently reading common types
pub struct StructIO {
    pub reader: Box<dyn Read>,
    pub is_le: bool, // The default endianness
}

macro_rules! read_type {
    ($func_name: ident, $type_: ty, default) => {
        pub fn $func_name(&mut self) -> io::Result<$type_> {
            let mut buffer = [0u8; size_of::<$type_>()];
            self.reader.read_exact(&mut buffer)?;
            if self.is_le {
                Ok(<$type_>::from_le_bytes(buffer))
            } else {
                Ok(<$type_>::from_be_bytes(buffer))
            }
        }
    };
    ($func_name: ident, $type_: ty, be) => {
        pub fn $func_name(&mut self) -> io::Result<$type_> {
            let mut buffer = [0u8; size_of::<$type_>()];
            self.reader.read_exact(&mut buffer)?;
            Ok(<$type_>::from_be_bytes(buffer))
        }
    };
    ($func_name: ident, $type_: ty, le) => {
        pub fn $func_name(&mut self) -> io::Result<$type_> {
            let mut buffer = [0u8; size_of::<$type_>()];
            self.reader.read_exact(&mut buffer)?;
            Ok(<$type_>::from_le_bytes(buffer))
        }
    };
}

impl StructIO {
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.reader.read_exact(buf)
    }

    // ----- Unsigned Int -----

    // Reads a u64 big endian
    read_type!(read_u64_be, u64, be);

    // Reads a u64 little endian
    read_type!(read_u64_le, u64, le);

    // Reads a u64 using default endianness
    read_type!(read_u64, u64, default);

    // Reads a u32 big endian
    read_type!(read_u32_be, u32, be);

    // Reads a u32 little endian
    read_type!(read_u32_le, u32, le);

    // Reads a u32 using default endianness
    read_type!(read_u32, u32, default);

    // Reads a u16 big endian
    read_type!(read_u16_be, u16, be);

    // Reads a u16 little endian
    read_type!(read_u16_le, u16, le);

    // Reads a u16 using default endianness
    read_type!(read_u16, u16, default);

    // Reads a u8 big endian
    read_type!(read_u8_be, u8, be);

    // Reads a u8 little endian
    read_type!(read_u8_le, u8, le);

    // Reads a u8 using default endianness
    read_type!(read_u8, u8, default);

    // ----- Signed Int -----

    // Reads a i64 big endian
    read_type!(read_i64_be, i64, be);

    // Reads a i64 little endian
    read_type!(read_i64_le, i64, le);

    // Reads a i64 using default endianness
    read_type!(read_i64, i64, default);

    // Reads a i32 big endian
    read_type!(read_i32_be, i32, be);

    // Reads a i32 little endian
    read_type!(read_i32_le, i32, le);

    // reads a i32 using default endianness
    read_type!(read_i32, i32, default);

    // Reads a i16 big endian
    read_type!(read_i16_be, i16, be);

    // Reads a i16 little endian
    read_type!(read_i16_le, i16, le);

    // reads a i16 using default endianness
    read_type!(read_i16, i16, default);

    // Reads a i8 big endian
    read_type!(read_i8_be, i8, be);

    // Reads a i8 little endian
    read_type!(read_i8_le, i8, le);

    // reads a i8 using default endianness
    read_type!(read_i8, i8, default);

    // ----- Float -----

    // Reads a f64 big endian
    read_type!(read_f64_be, f64, be);

    // Reads a f64 little endian
    read_type!(read_f64_le, f64, le);

    // reads a f64 using default endianness
    read_type!(read_f64, f64, default);

    // Reads a f32 big endian
    read_type!(read_f32_be, f64, be);

    // Reads a f32 little endian
    read_type!(read_f32_le, f64, le);

    // reads a f32 using default endianness
    read_type!(read_f32, f32, default);
}
