fn main() {
    // rust way of doing things
    let mut x = 100;

    let y = &mut x;

    *y = 200;


    // respecting rust borrow checker and having memory safety

    let mut x = 100;

    let p1: *const i32 = &x;

    unsafe {
        println!("{}", *p1);
    }

    let mut x = 100;

    let y1 = &mut x; // rust managed mutable reference

    // let y2 = &mut x;

    *y1 = 200;

    let mut x = 100;

    let y1: *mut i32 = &mut x; // raw mutable pointer

    let y2: *mut i32 = &mut x; // raw mutable pointer

    let y3: *mut bool = &mut true;

    let y4: *const i32 = std::ptr::null(); // only to the raw pointers

    unsafe {
        *y1 = 200;

        *y2 = 300;

        *y3 = false;

       // println!("{}", *y4);
    }

    println!("{}", x);

    let mut s1 = "hello world".to_string();

    let str2 = "hello Impact Analytics";
   
}

// rust references
// &T
// &mut T

// raw pointers
// * const i32 --> immutable
// * mut i32; --> mutable

// create fun --> take two integer references , give me the addition two values return a pointer
/*

    fn add_refs(a:&i32,b:&i32)->&i32{
    let mut r = 0;

    r = *a + *b ;

    return &r
    }

*/

// 1. References 
//      immutable and mutable &T &mut T
// 2. Smart Pointers 
//      Box, Arc, Rc , RefCell 

// 3. raw pointers 
//    - * const T
//    - *mut T

// 1 and 2 rust way of doing things, which is memory safety guarantees.
// 3 unsafe , that means you have to manage the allocations, deacllocations, nulls, dangalings, healp leaks ..everything
