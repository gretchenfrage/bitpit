
mod memory;

use memory::Memory;

fn main() {
    let mut mem = Memory::new();

    let mut i: i128 = 1;

    for _ in 0..100 {
        mem.set_bit(i, true);
        assert_eq!(mem.get_bit(i), true);

        println!("success with i={}", i);

        i *= 2;
    }

    println!("height = {}", mem.tree_layers());

    /*
    for i in 0..1000000 {
        if mem.get_bit(i) {
            println!("yes @ {}", i);
            return;
        }
    }

    for i in 0..10000000 {
        if i % 2 == 0 {
            //println!("insert {}", i);
            mem.set_bit(i, true);
        }

        assert_eq!(mem.get_bit(i), i % 2 == 0);

        if i % 100000 == 0 {
            println!("inserted {}", i);
        }
    }

    println!("tree height = {}", mem.tree_layers());

    //println!("none");
    */
}
