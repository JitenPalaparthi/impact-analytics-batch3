static mut COUNTER: i32 = 9999; // global variable -> data Segment
const PI: f32 = 3.14; // data segment / text code

fn main() {
    let (mut a, b, c) = (10, true, "Hello"); // stack memory
    let str1: &str = "Hello World"; // stack and DS
    a = 500;
    // b = false; // This does not work
    unsafe {
        COUNTER -= 1; // what is the problem using Global variables
        println!("{}", unsafe { COUNTER });
    }

    let s1: String = String::from("Hello World");

    let mut s2 = "Hello World".to_string();

    s2.push_str(" !how are you doing");

    let mut s3 = String::new();

    s3.push_str("Hello Impact Analytics");

    println!("str1: {:p} address of str1: {:p} address of a:{:p}",&str1,str1.as_ptr(),&a);
    println!("s1:{:p} s1:{:p}",&s1,s1.as_ptr());

    let s4 = String::from(str1);

        println!("s4:{:p} s4:{:p}",&s4,s4.as_ptr());


}

fn incr() {
    unsafe {
        COUNTER += 1;
    }
}

// What is in stack
// What is in Data Segment
