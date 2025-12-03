fn main() {
    let str1 = "Hello 你好 😃"; // &str

    println!("Length:{}",str1.len());
    for c in str1.chars(){
          print!("{} ",c);
    }
    println!();

    for b in str1.as_bytes(){
        print!("{:b} ",b);
    }

    // let num = b'100100;
    // let num = 0x'fff11;

}
