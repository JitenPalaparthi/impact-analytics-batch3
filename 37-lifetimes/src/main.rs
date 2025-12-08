static mut SQ: i32 = 0; // never be deallocated , lazyinit // Data Segment 

// BSS and RO 

static STR: &'static str = "Hello Impact Analytics"; // this is immutable internally

static SQ1:i32 = 100;

fn main() {
    let b1 = get_sq2(100);
    println!("Outside the box {:p}", b1.as_ref());
    let hello = get_str1();

    // let mut x = 100;
    // {
    //     let r = &mut x;
    // }

    // *r = 200;


    let r = get_sq3(100);
   
    println!("{:?}",unsafe{*r});

}

// fn get_sq1(n: i32) -> &i32 {
//     // dangling pointer
//     let mut sq = n * n; // variable is created here
//     return &sq; // life of sq ends
// }

// fn get_sq1(n: i32) -> &'static i32 {
//     // dangling pointer
//     let mut sq = n * n; // variable is created here
//     return &sq; // life of sq ends
// }

fn get_sq2(n: i32) -> Box<i32> {
    // box is creatd on stack and original data is on heap
    // it would extend the life time
    let mut sq = Box::new(n * n); // variable is created here 
    println!("Inside  the box {:p}", sq.as_ref());
    return sq; // life of sq ends
}

fn get_sq3(n: i32) -> *const i32 {
    // no problem with dangling pointer , bcz it is static lifetime
    unsafe {
        SQ = n * n;
        return SQ as *const i32;
    }
}


fn get_sq4(n: i32) -> &'static i32{
    // no problem with dangling pointer , bcz it is static lifetime
   &SQ1
}

fn get_str1() -> &'static str {
    "Hello World" // it is a string literal, it is stored in data segment 
}

// fn get_str2() -> &'static str {
//     let s = "Hello World".to_string(); // where is it allocated ? , on heap
//     return s.as_str(); // convert it to &str
//     // it is a string literal, it is stored in data segment
// }

fn get_str3() -> &'static str {
    STR
}




// Owned type vs Borrowed types
