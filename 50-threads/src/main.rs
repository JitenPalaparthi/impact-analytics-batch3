use std::{fmt::format, thread};
use tinyrand::{Rand, StdRand};
fn main() {
    let range = 10;
    let mut rand = StdRand::default();
    let handle1 = thread::spawn(move || {
        for i in 1..=range {
            let lrange = rand.next_lim_u32(100);
            let result = thread::spawn(move || -> String {
                let mut sum = 0;
                for i in 1..=lrange {
                    sum += i;
                }
                return format!("sum from a thread of range:{}-->{}", lrange, sum);
            })
            .join()
            .unwrap();
            println!("{}", result);
        }
    });

    handle1.join().unwrap();
    println!("Done with main.. exiting")
}

// loop of 10

// every time it should generate a rand number 100

// for every rand number , take a saperate loop in side a separate threads from 1-- rand number ..
// return the sum of numbers

//
