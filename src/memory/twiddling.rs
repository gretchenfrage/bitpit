
use std::ops;
use std::{i128, usize};

use super::node::PAGE_SIZE;

pub fn floor_div_rem<T>(a: T, b: T) -> (T, T)
    where
        T: Copy,
        T: ops::Rem<Output=T> + ops::Div<Output=T>,
        T: ops::Add<Output=T> + ops::Sub<Output=T>,
{

    let rem = ((a % b) + b) % b;
    let quo = (a - rem) / b;

    (quo, rem)
}

#[derive(Debug, Copy, Clone)]
pub struct UnpackedIndex {
    pub quo: i128,
    pub rem: usize,
    pub bit: u8,
}

impl UnpackedIndex {
    pub fn from_address_height(address: i128, height: u32) -> Self {
        let (word_quo, bit_rem) = floor_div_rem(address, 8);

        let words_per_child: i128 = (PAGE_SIZE as i128)
            .checked_pow(height)
            .expect("height too high");

        let (leaf_quo, word_rem) = floor_div_rem(word_quo, words_per_child);

        UnpackedIndex {
            quo: leaf_quo,
            rem: word_rem as usize,
            bit: bit_rem as u8,
        }
    }
}

pub fn get_word_bit(word: u8, bit_index: u8) -> bool {
    debug_assert!(bit_index < 8);
    (word & (0x1 << bit_index)) != 0x0
}

pub fn set_word_bit(word: &mut u8, bit_index: u8, bit: bool) {
    if bit {
        *word = *word | (0x1 << bit_index);
    } else {
        *word = *word & !(0x1 << bit_index);
    }
}
