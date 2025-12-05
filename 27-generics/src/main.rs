use std::ops::Add;
fn main() {
    let arr1 = [10, 34, 56, 12, 5, 45, 67, 145];

    let vec1 = vec![10, 34, 56, 12, 5, 45, 67, 145];

    let max = max_of(&arr1[..]);
    println!("max:{}", max);

    let max = max_of(&vec1);

    println!("max:{}", max);

    // let r = add_g(true,true);

    // let r = add_g('A','B');

    let sum = add_g(123.123, 12312.123);
    println!("sum:{}", sum);

    let sum = add_g(123, 12312);
    println!("sum:{}", sum);

    let sum = add_g(123u8, 122u8);
    println!("sum:{}", sum);

    let r1 = Rect::new(123.32, 432.23);

    let r2 = Rect::new(32.43, 1.4);

    let r3 = add_g(r1, r2);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add_g<T: std::ops::Add<Output = T>+Copy>(a: T, b: T) -> T {
    a + b // arithmetic operations,, cannot be performed on all types
}

fn add_g_w<T>(a: T, b: T) -> T // same as above function but different syntax where is used
where
    T: std::ops::Add<Output = T>,
{
    a + b // arithmetic operations,, cannot be performed on all types
}

#[derive(Debug, Copy,Clone)]
struct Rect {
    l: f32,
    b: f32,
}

impl Rect {
    fn new(l: f32, b: f32) -> Self {
        return Rect { l: l, b: b };
    }
}

impl Add for Rect {
    type Output = Rect; // asserted type
    fn add(self, rhs: Self) -> Self::Output {
        Rect {
            l: self.l + rhs.l,
            b: self.b + rhs.b,
        }
    }
}

