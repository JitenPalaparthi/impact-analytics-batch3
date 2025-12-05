// comptime
const fn get_sum(n: i32) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}

const K: i32 = get_sum(9);


#[no_mangle]
pub fn get_k() -> i32 {
    K
}

fn main() {
    println!("result: {}", K);  // prints 66
}

// rustc -O --emit asm src/main.rs