use std::thread;
static mut COUNTER: i32 = 0;
#[allow(static_mut_refs)]
fn main() {
    let handler1= thread::spawn(|| {
        for i in 1..=100 {
            unsafe {
                COUNTER += 1;
            }
        }
    });

    let handler2= thread::spawn(|| {
        for i in 1..=100 {
            unsafe {
                COUNTER -= 1;
            }
        }
    });

    handler1.join().unwrap();
    handler2.join().unwrap();

    unsafe{
        println!("{}",COUNTER);
    }
}
