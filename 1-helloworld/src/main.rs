fn main() {
    println!("Hello, world!"); // println! is a macro 

    let num1: i8 = 41; // life of variable

    let mut num2 = 123312; // type inference 

    let num2 =12312i64;
    //num2 = 987;

    let ok1 = true;

    let mut str1 = "Hello Impact analyticds!"; // string slice , string literal 

    let char1 = 'A';

    let char3: u8 = 'a' as u8;

    let char2 = '爱';

     let char4: u8 = '爱' as u8; // i byte from the whole char

    let num3: i64 = 1231231231231;

    let num4 = 1231231231231312312i64;

    let num5 = -1231231i32;

    let float1: f64 = 12312.12312312;

    let (a, b) = (10, 20);
    let (n1, s1, b1) = (1231, "Hello", true);
    let t1: (i32, &str, bool) = (1231, "Hello", true);

    println!(
        "num1:{} num2:{} ok1:{} str1:{} char1:{} char2:{} num3:{} num4:{} num5:{}",
        num1, num2, ok1, str1, char1, char2, num3, num4, num5
    );

    //num2 = 987878;

    str1 = "hello Impact Analytics Inc 爱 爱 ⚠️"; // when I am able to mutate ,why do you str is mutable 
    // collection of bytes
    println!("{}", str1);
    println!("float1:{:.3}", float1);

    println!("touple a:{} b:{}", a, b);

    println!("touple n1:{} s1:{} b1:{}", n1, s1, b1);

    println!("touple 0:{} 1:{} b2:{}", t1.0, t1.1, t1.2);

    let num1 = 123312312312312312312313112313i128; // not mutation, this is a new variable, probly with the same name

    {
        // stack frame
        let num1: i8 = 41; // life of variable
        let ok2 = true;
        println!("num1:{}", num1);
    }

    // println!("{}",ok2);
    println!("num1:{}", num1);
}

// numbers

// i8,i16,i32,i64,i128, u8,u16,u32,u64, isize, usize, f32, f64
// 32 bit os --> isize or usize if 4 bytes
// 64 -> 8 bytes

// bool - 1bye
// char - 4 bytes
// &str  - 16 bytes
// String -> 24 bytes
