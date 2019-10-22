
use std::{i128, usize};

use self::node::Node;

pub(self) mod node;

pub(self) mod twiddling;

mod read;

mod write;

#[derive(Clone)]
pub struct Memory {
    root: Box<Node>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            root: Box::new(Node::page(0))
        }
    }

    pub fn get_bit(&self, address: i128) -> bool {
        read::search_bit(self.root.as_ref(), address)
            .unwrap_or(false)
    }

    pub fn set_bit(&mut self, address: i128, bit: bool) {
        write::insert_bit(&mut self.root, address, bit);
    }

    pub fn tree_layers(&self) -> usize {
        self.root.layers()
    }
}