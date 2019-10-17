
use std::{i128, usize};
use std::mem::{size_of, self};
use std::ops;

use self::node::Node;

pub(self) mod node;

pub(self) mod twiddling;

mod read;

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
}


/*
   pub fn set_bit(&mut self, address: i128, bit: bool) {
       let mut curr: &mut Box<Node> = &mut self.root;

       'down: loop {

           match curr.as_mut() {
               &mut Node::Page {
                   page_index,
                   ref mut bits,
               } => {
                   let UnpackedIndex {
                       curr_level_index,
                       child_index,
                       bit_index,
                   } = unpack_index(address, 0);


               }
           };

       }
   }
   */

/*
pub fn set_bit(&mut self, address: i128, bit: bool) {
    enum LoopEffect<'c> {
        Descend(&'c mut Box<Node>),
        Ascend {
            curr_parent_magnitude: u32,
            curr_reattach_div_rem: (i128, i128),
        },
    }

    let mut curr: &mut Box<Node> = &mut self.root;
    'seek: loop {
        let effect: LoopEffect = match curr.as_mut() {
            &mut Node::Page {
                page_index,
                ref mut bits,
            } => {
                let UnpackedIndex {
                    curr_level_index,
                    child_index,
                    bit_index,
                } = unpack_index(address, 0);

                let effect: LoopEffect = match curr_level_index == page_index {
                    true => {
                        set_bit(&mut bits[child_index], bit_index, bit);
                        return;
                    },
                    false => LoopEffect::Ascend {
                        curr_parent_magnitude: 1,
                        curr_reattach_div_rem: floor_div_rem(
                            page_index,
                            PAGE_SIZE as i128
                        ),
                    },
                };
                effect
            },

            &mut Node::Branch {
                magnitude_order,
                magnitude_relative_index,
                ref mut children,
            } => {
                let UnpackedIndex {
                    curr_level_index,
                    child_index,
                    bit_index,
                } = unpack_index(address, 0);

                let effect: LoopEffect = match curr_level_index == magnitude_relative_index {
                    true => {
                        let child_ptr: &mut Option<Box<Node>> = &mut children[child_index];

                        if child_ptr.is_none() {
                            let node: Node = Node::new_page_containing_address(address);
                            *child_ptr = Some(Box::new(node));
                        };

                        LoopEffect::Descend(child_ptr.as_mut().unwrap())
                    },
                    false => LoopEffect::Ascend {
                        curr_parent_magnitude: magnitude_order + 1,
                        curr_reattach_div_rem: floor_div_rem(
                            magnitude_relative_index,
                            PAGE_SIZE as i128
                        ),
                    },
                };
                effect
            },
        };

        let (
            mut curr_parent_magnitude,
            mut curr_reattach_div_rem,
        ) = match effect {
            LoopEffect::Descend(child) => {
                curr = child;
                continue 'seek;
            },
            LoopEffect::Ascend {
                mut curr_parent_magnitude,
                mut curr_reattach_div_rem,
            } => (
                curr_parent_magnitude,
                curr_reattach_div_rem,
            ),
        };

        let next: &mut Box<Node> = {
            let new_attach_index: usize = 'raise: loop {
                let UnpackedIndex {
                    curr_level_index: possible_attach_div,
                    child_index: possible_attach_rem,
                    ..
                } = unpack_index(address, curr_parent_magnitude);

                if possible_attach_div == curr_reattach_div_rem.0 {
                    break 'raise possible_attach_rem;
                } else {
                    curr_parent_magnitude += 1;
                    curr_reattach_div_rem = floor_div_rem(
                        curr_reattach_div_rem.0,
                        PAGE_SIZE as i128,
                    );
                }
            };

            let new_parent: Node = Node::Branch {
                magnitude_order: curr_parent_magnitude,
                magnitude_relative_index: curr_reattach_div_rem.0,
                children: empty_children_array(),
            };
            let new_parent: Box<Node> = Box::new(new_parent);

            let to_reattach: Box<Node> = mem::replace(
                curr,
                new_parent,
            );

            if let &mut Node::Branch {
                ref mut children,
                ..
            } = curr.as_mut() {
                children[curr_reattach_div_rem.1 as usize] = Some(to_reattach);
                children[new_attach_index] = Some(Box::new(
                    Node::new_page_containing_address(address)
                ));

                children[new_attach_index].as_mut().unwrap()
            } else {
                unreachable!()
            }
        };

        curr = next;

        /*
        let next: &mut Box<Node> = match effect {
            LoopEffect::Descend(child) => child,
            LoopEffect::Ascend {
                mut curr_parent_magnitude,
                mut curr_reattach_div_rem,
            } => {

                let new_attach_index: usize = 'raise: loop {
                    let UnpackedIndex {
                        curr_level_index: possible_attach_div,
                        child_index: possible_attach_rem,
                        ..
                    } = unpack_index(address, curr_parent_magnitude);

                    if possible_attach_div == curr_reattach_div_rem.0 {
                        break 'raise possible_attach_rem;
                    } else {
                        curr_parent_magnitude += 1;
                        curr_reattach_div_rem = floor_div_rem(
                            curr_reattach_div_rem.0,
                            PAGE_SIZE as i128,
                        );
                    }
                };

                let new_parent: Node = Node::Branch {
                    magnitude_order: curr_parent_magnitude,
                    magnitude_relative_index: curr_reattach_div_rem.0,
                    children: empty_children_array(),
                };
                let new_parent: Box<Node> = Box::new(new_parent);

                let to_reattach: Box<Node> = mem::replace(
                    curr,
                    new_parent,
                );

                if let &mut Node::Branch {
                    ref mut children,
                    ..
                } = curr.as_mut() {
                    children[curr_reattach_div_rem.1 as usize] = Some(to_reattach);
                    children[new_attach_index] = Some(Box::new(
                        Node::new_page_containing_address(address)
                    ));

                    children[new_attach_index].as_mut().unwrap()
                } else {
                    unreachable!()
                }

            }
        };

        curr = next;
        */
    }
}
*/