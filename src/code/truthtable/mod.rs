
use std::borrow::{Borrow, BorrowMut};
use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

impl<T: Borrow<u8>> IoTruthTable<T> {
    pub fn bitwise_lookup(&self, input: bool, output: bool) -> bool {
        let shift: u8 = output as u8 + (input as u8 * 2);
        let mask: u8 = 0x1 << shift;

        // before comparing with 0x00, we mask out the higher order 4 bits
        // because we only care about the lower order 4 bits
        // the higher ones are allowed to be whatever
        ((self.0.borrow() & mask) & 0x0F) != 0x00
    }

    pub fn unpack(&self) -> IoTruthTable<TruthTableArray> {
        IoTruthTable([
            [
                self.bitwise_lookup(false, false),
                self.bitwise_lookup(false, true)
            ],
            [
                self.bitwise_lookup(true, false),
                self.bitwise_lookup(true, true)
            ],
        ])
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

    pub fn repack_from(&mut self, table: &IoTruthTable<TruthTableArray>) {
        self.bitwise_zero();
        self.bitwise_imprint(table);
    }

    pub fn copy_from<O>(&mut self, source: IoTruthTable<O>)
        where
            O: Borrow<u8> {

        let target: &mut u8 = self.0.borrow_mut();
        let source: &u8 = source.0.borrow();

        *target = *source;
    }
}

// bitwise operations on bit fields

use std::ops::{
    BitAnd, BitOr, BitXor,
    BitAndAssign, BitOrAssign, BitXorAssign,
    Not,
};

macro_rules! impl_bitfield_op {
    ($op:tt::$method:tt) => {
        impl<A, B> $op<IoTruthTable<B>> for IoTruthTable<A>
            where
                A: Borrow<u8>,
                B: Borrow<u8>, {

            type Output = IoTruthTable<u8>;

            fn $method(self, rhs: IoTruthTable<B>) -> Self::Output {
                let ref_a: &u8 = self.0.borrow();
                let ref_b: &u8 = rhs.0.borrow();

                IoTruthTable($op::$method(
                    *ref_a,
                    *ref_b,
                ))
            }
        }
    }
}

impl_bitfield_op!(BitAnd::bitand);
impl_bitfield_op!(BitOr::bitor);
impl_bitfield_op!(BitXor::bitxor);

macro_rules! impl_bitfield_op_assign {
    ($op:tt::$method:tt) => {
        impl<A, B> $op<IoTruthTable<B>> for IoTruthTable<A>
            where
                A: BorrowMut<u8>,
                B: Borrow<u8>, {

            fn $method(&mut self, rhs: IoTruthTable<B>) {
                let ref_a: &mut u8 = self.0.borrow_mut();
                let ref_b: &u8 = rhs.0.borrow();

                $op::$method(
                    ref_a,
                    *ref_b,
                )
            }
        }
    }
}

impl_bitfield_op_assign!(BitAndAssign::bitand_assign);
impl_bitfield_op_assign!(BitOrAssign::bitor_assign);
impl_bitfield_op_assign!(BitXorAssign::bitxor_assign);

impl<T: Borrow<u8>> Not for IoTruthTable<T> {
    type Output = IoTruthTable<u8>;

    fn not(self) -> Self::Output {
        let field: &u8 = self.0.borrow();
        IoTruthTable(Not::not(*field))
    }
}

// pretty-printing

use std::fmt::{self, Debug, Formatter};

fn pretty_format_io_function(table: &TruthTableArray) -> String {
    format!(
        "(i,o)={{ (f,f):{} (t,f):{} (f,t):{} (t,t):{} }}",
        table[0][0],
        table[1][0],
        table[0][1],
        table[1][1],
    )
}

impl Debug for IoTruthTable<TruthTableArray> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&format!(
            "IoTruthTable({})",
            pretty_format_io_function(&self.0),
        ))
    }
}

macro_rules! impl_truth_table_bitfield_debug {
    ($t:ty) => {
        impl Debug for IoTruthTable<$t> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.debug_struct("IoTruthTable")
                    .field("bitfield", &format!("{:#010b}", *Borrow::<u8>::borrow(&self.0)))
                    .field("unpacked", &self.unpack())
                    .finish()
            }
        }
    }
}

impl_truth_table_bitfield_debug!(u8);
impl_truth_table_bitfield_debug!(&u8);
impl_truth_table_bitfield_debug!(&mut u8);