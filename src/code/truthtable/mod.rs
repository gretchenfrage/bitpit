
use std::borrow::{Borrow, BorrowMut};
use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct IoTruthTable<T>(pub T);

// input is outer array.
// output is inner array.
pub type TruthTableArray = [[bool; 2]; 2];

impl Index<usize> for IoTruthTable<TruthTableArray> {
    type Output = [bool; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IoTruthTable<TruthTableArray> {
    pub fn no_unconditional() -> Self {
        IoTruthTable([
            [false, false],
            [false, false],
        ])
    }

    pub fn yes_unconditional() -> Self {
        IoTruthTable([
            [true, true],
            [true, true],
        ])
    }

    pub fn input_conditional() -> Self {
        IoTruthTable([
            [false, false],
            [true, true],
        ])
    }

    pub fn output_conditional() -> Self {
        IoTruthTable([
            [false, true],
            [false, true],
        ])
    }

    pub fn pack_bitfield(&self) -> IoTruthTable<u8> {
        let mut field = IoTruthTable::new_zeroed_bitfield();
        field.bitwise_imprint(&self);
        field
    }
}

impl IoTruthTable<u8> {
    pub fn new_zeroed_bitfield() -> Self {
        IoTruthTable(0x00)
    }
}

impl<T: BorrowMut<u8>> IoTruthTable<T> {
    pub fn bitwise_zero(&mut self) {
        *self.0.borrow_mut() = 0;
    }

    pub fn bitwise_imprint(&mut self, table: &IoTruthTable<TruthTableArray>) {
        let field: &mut u8 = self.0.borrow_mut();

        for i in 0_usize..2 {
            for o in 0_usize..2 {

                let shift: usize = o + (i * 2);
                let mask: u8 = 0x1 << shift as u8;

                if table[i][o] {
                    *field |= mask;
                }

            }
        }
    }
}

/*
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct InputOutputValues(pub bool, pub bool);

impl InputOutputValues {
    pub fn input(&self) -> bool {
        self.0
    }

    pub fn output(&self) -> bool {
        self.1
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct InputOutputTableUnpacked {
    input_no_output_no: bool,
    input_yes_output_no: bool,
    input_no_output_yes: bool,
    input_yes_output_yes: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct InputOutputTableBits<B>(pub B);

impl InputOutputTableBits<u8> {
    pub fn unconditional_no() -> Self {
        InputOutputTableBits(0x00)
    }

    pub fn unconditional_yes() -> Self {
        InputOutputTableBits(0x0F)
    }

    pub fn yes_when_input_yes() -> Self {
        let mut field = Self::unconditional_no();

    }
}

impl<B: Borrow<u8>> InputOutputTableBits<B> {

}

impl<B: BorrowMut<u8>> InputOutputTableBits<B> {

}

pub fn input_output_bit_offset(input: bool, output: bool) -> usize {
    match (input, output) {
        (false, false) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (true, true) => 3,
    }
}
*/