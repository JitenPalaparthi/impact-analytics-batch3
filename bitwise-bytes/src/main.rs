fn main() {
    let b1: u8 = 0b1100_1100;
    println!("{}", b1);

    for i in 1..100 {
        if i % 2 == 1 {
            continue;
        }
        println!("{:08b}", i);
    }

    let b1: u8 = 0b1111_1111;

    let b2: u16 = 0b1111_1111_1111_1111;

    println!("{:b} {:x} {}", b1, b1, b1);
    println!("{:b} {:x} {}", b2, b2, b2);

    let b1 = 254u8;

    let b2 = b1 & 0xff;
    let b3 = b1 | 0xff;
    let b4 = b1 ^ 0xff;

    println!("{}", b2);
    println!("{}", b3);
    println!("{}", b4);

    println!("{:08b}", 1 << 8);

    let x: u8 = 0b1111_1111;

    let k: u8 = 3;

    let p: u8 = 1 << 3;

    let is: u8 = (x & p);

    println!("x-->{:08b}", x);

    println!("k-->{:08b}", k);

    println!("p-->{:08b}", p);

    println!("i-->{:08b}", is);
}
