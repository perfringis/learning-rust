fn main() {
    // scalar type
    // A scalar type represents a single value
    // rust has 4 scalar types:
    // integer, floating-point number, boolean, character

    // isize and usize depend on architecture can store 64bits or 32bits

    // signed value stores from -(2^n-1) to 2^(n-1)-1
    // The example: i8 = from -128 to 127
    let signed_first: i8 = 127;
    let signed_second: i16 = -12;
    let signed_third: i32 = -12;
    let signed_fourth: i64 = -12;
    let signed_fifth: i128 = -12;
    let signed_sixth: isize = -12;

    println!("value {signed_first}");
    println!("value {signed_second}");
    println!("value {signed_third}");
    println!("value {signed_fourth}");
    println!("value {signed_fifth}");
    println!("value {signed_sixth}");

    // unsigned values only stores from 0 to 2^n - 1
    // The example: u8 = from 0 to 2^8 -1 = 255
    let unsigned_first: u8 = 12;
    let unsigned_second: u16 = 12;
    let unsigned_third: u32 = 12;
    let unsigned_fourth: u64 = 12;
    let unsigned_fifth: u128 = 12;
    let unsigned_sixth: usize = 12;

    println!("value {unsigned_first}");
    println!("value {unsigned_second}");
    println!("value {unsigned_third}");
    println!("value {unsigned_fourth}");
    println!("value {unsigned_fifth}");
    println!("value {unsigned_sixth}");

    let number_literal: u32 = 98_222;
    let hex_literal: u32 = 0xff;
    let octal_literal: u32 = 0o77;
    let binary_literal:u8 = 0b0000_1111;
    let binary_literal_2:u8 = b'A';

    println!("value {number_literal}");
    println!("value {hex_literal}");
    println!("value {octal_literal}");
    println!("value {binary_literal}");
    println!("value {binary_literal_2}");
    
    // in release mode compiler doesn't handle integer overflow
    // if assigned value exceeded stored value then at the end of the day it is not what we are expecting
}
