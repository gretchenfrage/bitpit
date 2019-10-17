
use std::{i128, usize};
use std::mem::{size_of, transmute, self};
use std::ptr;
use super::twiddling::floor_div_rem;

pub const PAGE_SIZE: usize = 0x400;
pub const BRANCH_FACTOR: usize = PAGE_SIZE / size_of::<usize>();

#[derive(Clone)]
pub enum Node {
    Page {
        index: i128,
        bits: [u8; PAGE_SIZE],
    },
    Branch {
        height: u32,
        index: i128,
        children: [Option<Box<Node>>; BRANCH_FACTOR],
    },
}

pub fn new_children() -> [Option<Box<Node>>; BRANCH_FACTOR] {
    unsafe {
        let equivalent: [*mut Node; BRANCH_FACTOR] = [ptr::null_mut(); BRANCH_FACTOR];

        transmute::<
            [*mut Node; BRANCH_FACTOR],
            [Option<Box<Node>>; BRANCH_FACTOR]
        >(equivalent)
    }
}

impl Node {
    pub fn page(index: i128) -> Self {
        Node::Page {
            index,
            bits: [0x00; PAGE_SIZE],
        }
    }

    pub fn page_containing_address(address: i128) -> Self {
        let (index, _) = floor_div_rem(address, PAGE_SIZE as i128);
        Node::page(index)
    }
}