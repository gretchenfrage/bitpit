
use super::node::*;
use super::twiddling::*;

use std::mem;

pub fn insert_bit(root: &mut Box<Node>, address: i128, bit: bool) {
    let mut curr: &mut Box<Node> = root;
    loop {
        // ascend
        ascend(curr, address);

        // descend
        let descend_to: usize = match Box::as_mut(curr) {

            &mut Node::Page {
                ref mut bits,
                row_index: page_row_index,
            } => {
                debug_assert_eq!(page_row_index, row_index(address, PageLevel));

                let word_index = child_index(address, WordLevel);
                let bit_index = child_index(address, BitLevel) as u8;

                let word: &mut u8 = &mut bits[word_index];
                set_word_bit(word, bit_index, bit);

                return;
            }

            &mut Node::Branch {
                level,
                children: _,
                row_index: branch_row_index,
            } => {
                debug_assert_eq!(branch_row_index, row_index(address, level));

                child_index(address, ChildOfBranchLevel(level))
            }

        };

        // descend part #2 (because lifetime limitations)
        if let &mut Node::Branch {
            ref mut children,
            ..
        } = Box::as_mut(curr) {

            curr = children[descend_to]
                .get_or_insert_with(|| {
                    Box::new(Node::page_containing_address(address))
                });

        } else {
            unreachable!()
        }
    }
}

fn ascend(node: &mut Box<Node>, address: i128) {
    if node.row_index() != node.row_index_of_address(address) {

        const FACTOR: i128 = BRANCH_FACTOR as i128;

        let mut level: BranchLevel = node.parent_level();
        let mut row_index: i128 = floor_div(node.row_index(), FACTOR);
        let mut reattach_index: i128 = floor_rem(node.row_index(), FACTOR);

        while reattach_index != child_index(address, level) as i128 {
            level = level.parent();
            row_index = floor_div(row_index, FACTOR);
            reattach_index /= FACTOR;
        }

        let parent: Box<Node> = Box::new(Node::Branch {
            level,
            row_index,
            children: new_children(),
        });
        let child: Box<Node> = mem::replace(node, parent);

        if let &mut Node::Branch {
            ref mut children,
            ..
        } = Box::as_mut(node) {

            children[reattach_index as usize] = Some(child);

        } else {
            unreachable!()
        }

    }
}
