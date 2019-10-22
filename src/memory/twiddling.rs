use std::ops;

pub fn floor_rem<T>(a: T, b: T) -> T
    where
        T: Copy,
        T: ops::Rem<Output=T> + ops::Div<Output=T>,
        T: ops::Add<Output=T> + ops::Sub<Output=T>, {
    ((a % b) + b) % b
}

pub fn floor_div_rem<T>(a: T, b: T) -> (T, T)
    where
        T: Copy,
        T: ops::Rem<Output=T> + ops::Div<Output=T>,
        T: ops::Add<Output=T> + ops::Sub<Output=T>, {
    let rem = floor_rem(a, b);
    let quo = (a - rem) / b;

    (quo, rem)
}

pub fn floor_div<T>(a: T, b: T) -> T
    where
        T: Copy,
        T: ops::Rem<Output=T> + ops::Div<Output=T>,
        T: ops::Add<Output=T> + ops::Sub<Output=T>, {
    floor_div_rem(a, b).0
}

pub fn get_word_bit(word: u8, bit_index: u8) -> bool {
    debug_assert!(bit_index < 8);

    (word & (0x1 << bit_index)) != 0x0
}

pub fn set_word_bit(word: &mut u8, bit_index: u8, bit: bool) {
    debug_assert!(bit_index < 8);

    if bit {
        *word = *word | (0x1 << bit_index);
    } else {
        *word = *word & !(0x1 << bit_index);
    }
}
