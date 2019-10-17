
use std::{i128, usize};
use std::mem::{size_of, self};
use std::ops;

const PAGE_SIZE: usize = 0x400;

#[derive(Clone)]
pub struct Memory {
    root: Box<Node>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            root: Box::new(Node::new_page(0))
        }
    }

    pub fn get_bit(&self, address: i128) -> bool {
        self.root.get(address).unwrap_or(false)
    }
}

#[derive(Clone)]
enum Node {
    Page {
        page_index: i128,
        bits: [u8; PAGE_SIZE],
    },
    Branch {
        magnitude_order: u32,
        magnitude_relative_index: i128,
        children: [Option<Box<Node>>; PAGE_SIZE / size_of::<usize>()],
    },
}

impl Node {
    fn new_page(page_index: i128) -> Self {
        Node::Page {
            page_index,
            bits: [0x00; PAGE_SIZE],
        }
    }

    fn new_page_containing_address(address: i128) -> Self {
        let UnpackedIndex {
            curr_level_index: page_index,
            ..
        } = unpack_index(address, 0);
        Node::new_page(page_index)
    }
}

fn empty_children_array() -> [Option<Box<Node>>; PAGE_SIZE / size_of::<usize>()] {
    unsafe {
        use std::mem::transmute;
        use std::ptr;

        const LEN: usize = PAGE_SIZE / size_of::<usize>();

        let equivalent: [*mut Node; LEN] = [ptr::null_mut(); LEN];

        transmute(equivalent)
    }
}

#[derive(Debug, Copy, Clone)]
struct UnpackedIndex {
    curr_level_index: i128,
    child_index: usize,
    bit_index: u8,
}

fn unpack_index(address: i128, magnitude: u32) -> UnpackedIndex {
    let (word_index, bit_index) = floor_div_rem(address, 8);

    let words_per_child: i128 = (PAGE_SIZE as i128)
        .checked_pow(magnitude)
        .expect("magnitude too high");

    let (curr_level_index, child_index) = floor_div_rem(word_index, words_per_child);

    UnpackedIndex {
        curr_level_index,
        child_index: child_index as usize,
        bit_index: bit_index as u8,
    }
}

/*
fn unpack_index(address: i128, magnitude: u32) -> UnpackedIndex {
    let bit_index: u8 = (address % 8) as u8;

    // always round down on int division
    let word_index: i128 = match bit_index {
        0 => address / 8,
        _ => match address >= 0 {
            true => address / 8,
            false => (address / 8) - 1,
        },
    };

    let words_per_child: i128 = (PAGE_SIZE as i128)
        .checked_pow(magnitude)
        .expect("magnitude too high");

    let child_index: usize = (word_index % words_per_child) as usize;

    // always round down on int division
    let curr_level_index: i128 = match child_index {
        0 => word_index / words_per_child,
        _ => match word_index >= 0 {
            true => word_index / words_per_child,
            false => (word_index / words_per_child) - 1,
        },
    };

    UnpackedIndex {
        curr_level_index,
        child_index,
        bit_index,
    }
}
*/

impl Memory {
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
}

impl Node {
    fn get(&self, address: i128) -> Option<bool> {
        let mut curr: &Node = self;
        loop {
            let next: &Node = match curr {
                &Node::Page {
                    page_index,
                    ref bits,
                } => {
                    let UnpackedIndex {
                        curr_level_index,
                        child_index,
                        bit_index,
                    } = unpack_index(address, 0);

                    let bit: Option<bool> = match curr_level_index == page_index {
                        true => Some(get_bit(bits[child_index], bit_index)),
                        false => None,
                    };
                    return bit;
                },

                &Node::Branch {
                    magnitude_order,
                    magnitude_relative_index,
                    ref children,
                } => {
                    debug_assert!(magnitude_order > 0);

                    let UnpackedIndex {
                        curr_level_index,
                        child_index,
                        bit_index,
                    } = unpack_index(address, magnitude_order);

                    let child: &Node = match curr_level_index == magnitude_relative_index {
                        true => match &children[child_index] {
                            Some(child) => child.as_ref(),
                            None => {
                                return None;
                            }
                        },
                        false => {
                            return None;
                        },
                    };
                    child
                },
            };

            curr = next;
        }
    }
}

fn floor_div_rem<T>(a: T, b: T) -> (T, T)
    where
        T: Copy,
        T: ops::Rem<Output=T> + ops::Div<Output=T>,
        T: ops::Add<Output=T> + ops::Sub<Output=T>,
{

    let rem = ((a % b) + b) % b;
    let quo = (a - rem) / b;

    (quo, rem)
}

fn get_bit(word: u8, bit_index: u8) -> bool {
    debug_assert!(bit_index < 8);
    (word & (0x1 << bit_index)) != 0x0
}

fn set_bit(word: &mut u8, bit_index: u8, bit: bool) {
    if bit {
        *word = *word | (0x1 << bit_index);
    } else {
        *word = *word & !(0x1 << bit_index);
    }
}
