
use std::{i128, usize};
use std::mem::{size_of, transmute, self};
use std::ptr;
use super::twiddling::*;

//pub const PAGE_SIZE: usize = 0x400;
//pub const BRANCH_FACTOR: usize = PAGE_SIZE / size_of::<usize>();

pub const PAGE_SIZE: usize = 2;
pub const BRANCH_FACTOR: usize = 2;

#[derive(Clone)]
pub enum Node {
    Page {
        row_index: i128,
        bits: [u8; PAGE_SIZE],
    },
    Branch {
        level: BranchLevel,
        row_index: i128,
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
    pub fn page(row_index: i128) -> Self {
        Node::Page {
            row_index,
            bits: [0x00; PAGE_SIZE],
        }
    }

    pub fn page_containing_address(address: i128) -> Self {
        let row_index = row_index(address, PageLevel);
        Node::page(row_index)
    }

    pub fn row_index(&self) -> i128 {
        match self {
            &Node::Page { row_index, .. } => row_index,
            &Node::Branch { row_index, .. } => row_index,
        }
    }

    pub fn row_index_of_address(&self, address: i128) -> i128 {
        match self {
            &Node::Page { .. } => row_index(address, PageLevel),
            &Node::Branch { level, .. } => row_index(address, level),
        }
    }
}

pub trait TreeLevel {
    type Parent: TreeLevel;

    fn scale_factor(&self) -> i128;

    fn parent(&self) -> Self::Parent;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BitLevel;
impl TreeLevel for BitLevel {
    type Parent = WordLevel;

    fn scale_factor(&self) -> i128 {
        1
    }

    fn parent(&self) -> Self::Parent {
        WordLevel
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WordLevel;
impl TreeLevel for WordLevel {
    type Parent = PageLevel;

    fn scale_factor(&self) -> i128 {
        8
    }

    fn parent(&self) -> Self::Parent {
        PageLevel
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PageLevel;
impl TreeLevel for PageLevel {
    type Parent = BranchLevel;

    fn scale_factor(&self) -> i128 {
        8 * PAGE_SIZE as i128
    }

    fn parent(&self) -> Self::Parent {
        BranchLevel(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BranchLevel(pub u32);
impl TreeLevel for BranchLevel {
    type Parent = BranchLevel;

    fn scale_factor(&self) -> i128 {
        8 * PAGE_SIZE as i128 * (BRANCH_FACTOR as i128).pow(self.0 + 1)
    }

    fn parent(&self) -> Self::Parent {
        BranchLevel(self.0 + 1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ChildOfBranchLevel(pub BranchLevel);
impl TreeLevel for ChildOfBranchLevel {
    type Parent = BranchLevel;

    fn scale_factor(&self) -> i128 {
        8 * PAGE_SIZE as i128 * (BRANCH_FACTOR as i128).pow((self.0).0)
    }

    fn parent(&self) -> Self::Parent {
        self.0
    }
}

pub fn row_index<T: TreeLevel>(address: i128, level: T) -> i128 {
    floor_div(address, level.scale_factor())
}

pub fn child_index<T: TreeLevel>(address: i128, level: T) -> usize {
    floor_div(
        floor_rem(address, level.parent().scale_factor() as i128),
        level.scale_factor() as i128,
    ) as usize
}