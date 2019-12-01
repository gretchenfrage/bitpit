
use super::TokenTree;
use crate::code::span::Spanned;

/// Debug-print a slice of token trees.
pub fn print_tt(
    tt: &[TokenTree],
    print_spans: bool,
) {
    // stack of layers, starts with base
    let mut stack: Vec<&[TokenTree]> = vec![tt];

    // iterate until exhausted
    while let Some(layer) = stack.pop() {
        // if there is an element, handle and re-insert
        if let Some(first) = layer.get(0) {
            let rest = &layer[1..];

            match first {
                &TokenTree::Token(Spanned(token, span)) => {
                    // if we hit a token, print it
                    for _ in 0..stack.len() {
                        print!("  ");
                    }
                    print!("- ");
                    println!("{:?}", token);

                    if print_spans {
                        for _ in 0..stack.len() {
                            print!("  ");
                        }
                        println!("  {:?}", span)
                    }

                    // the push back the remainder
                    stack.push(rest);
                },

                &TokenTree::ParenScope(ref sublayer) => {
                    // if we hit a sub-scope, push it OVER the remainder
                    stack.push(rest);
                    stack.push(Vec::as_slice(sublayer));
                }
            }

        }
        // if that layer was exhausted, then it will not be re-inserted
    }
}
