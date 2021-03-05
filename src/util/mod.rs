use std::fmt::{Display, LowerHex};
use std::ops::{Add, Mul};

pub fn print_buffer_as_hex<T: LowerHex>(buffer: &[T]) {
    for item in buffer.iter() {
        print!("0x{} ", format!("{:x}", item))
    }
}

pub fn print_buffer<T: Display>(buffer: &[T]) {
    for item in buffer.iter() {
        print!("{} ", item)
    }
}

pub fn get_decimal_sum_from_buffer(buffer: &[u8]) -> u128 {
    let length = buffer.len();
    let mut sum: u128 = 0;
    let base: u128 = 256;
    for (index, item) in buffer.iter().enumerate() {
        sum = sum + ((item.clone() as u128 * base.pow((length - 1 - index) as u32)) as u128)
    }

    sum
}
