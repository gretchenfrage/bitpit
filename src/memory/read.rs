
use super::node::*;
use super::twiddling::*;

pub fn search_bit(root: &Node, address: i128) -> Option<bool> {
    let mut curr: &Node = root;
    loop {
        let next: &Node = match curr {
            &Node::Page {
                index: curr_index,
                bits: ref bitfield,
            } => {
                let UnpackedIndex {
                    quo: page_index,
                    rem: word_index,
                    bit: bit_index,
                } = UnpackedIndex::from_address_height(address, 0);

                if curr_index == page_index {
                    return Some(
                        get_word_bit(bitfield[word_index], bit_index)
                    );
                } else {
                    return None;
                }
            },

            &Node::Branch {
                height: curr_height,
                index: curr_index,
                ref children,
            } => {
                debug_assert!(curr_height > 0);

                let UnpackedIndex {
                    quo: branch_index,
                    rem: child_index,
                    bit: _,
                } = UnpackedIndex::from_address_height(address, curr_height);

                if curr_index == branch_index {
                    if let &Some(ref child) = &children[child_index] {
                        Box::as_ref(child)
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },
        };

        curr = next;
    }
}