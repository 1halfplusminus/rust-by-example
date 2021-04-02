#![allow(overflowing_literals)]

use std::u8;
fn main() {
    let decimal = 65.4312_f32;

    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);

    println!("1000 mod 250 is : {}", 1000 % 250);

    println!(" 120 as a i16 is: {}", 128 as i16);

    println!(" 120 as i8 is: {}", 128 as i8);

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());

        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
    }
}
