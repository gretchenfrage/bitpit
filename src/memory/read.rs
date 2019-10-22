
use super::node::*;
use super::twiddling::*;

pub fn search_bit(root: &Node, address: i128) -> Option<bool> {
    let mut curr: &Node = root;
    loop {

        if curr.row_index() == curr.row_index_of_address(address) {
            match curr {

                &Node::Page {
                    ref bits,
                    ..
                } => {
                    let word_index = child_index(address, WordLevel);
                    let bit_index = child_index(address, BitLevel) as u8;

                    let word = bits[word_index];
                    let bit = get_word_bit(word, bit_index);

                    return Some(bit);
                }

                &Node::Branch {
                    level,
                    row_index: _,
                    ref children,
                } => {
                    let i = child_index(address, ChildOfBranchLevel(level));

                    if let &Some(ref node) = &children[i] {
                        curr = Box::as_ref(node);
                    } else {
                        return None;
                    }
                }

            }
        } else {
            return None;
        }

    }
}