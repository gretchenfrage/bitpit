
mod memory;

use memory::Memory;

fn main() {
    let mut mem = Memory::new();

    for i in 0..1000000 {
        if mem.get_bit(i) {
            println!("yes @ {}", i);
            return;
        }
    }

    for i in 0..1000000 {
        if i % 2 == 0 {
            println!("insert {}", i);
            mem.set_bit(i, true);
        }

        assert_eq!(mem.get_bit(i), i % 2 == 0);
    }

    println!("none");
}
