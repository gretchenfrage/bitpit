
mod memory;

use memory::Memory;

fn main() {
    let mut mem = Memory::new();

    for i in 0..10000000 {
        if mem.get_bit(i) {
            println!("yes @ {}", i);
            return;
        }
    }

    println!("none");
}
