use std::ops::{Mul,Add};
fn main() {

    let x:i32 = 100; // Into

    let y:i64= i64::from(x); // From

    let r1 = Rect::new(12.3f32,14.56f32);

    // T --> f32

    // area -> f64

    let a1= r1.area::<f64>();
    let a1:f64 = r1.area();
    println!("area of r1:{}",a1);



     let r1 = Rect::new(12,14);

    // T --> f32

    // area -> f64

    let a1= r1.area::<i64>();
    let a1:f64 = r1.area();
    println!("area of r1:{}",a1);

    let b = Box::new(10);
    let b= Box::<i32>::new(10);

    let v = Vec::<i32>::new();
    let mut v = Vec::new();
    v.push(10);

    println!("{:?}",v);
}

#[derive(Debug)]
struct Rect<T> {
    l: T,
    b: T,
}

impl<T> Rect<T> where T:Mul<Output=T>+Copy {
    fn new(l: T, b: T) -> Self {
        return Rect { l: l, b: b };
    }
    fn area<U>(&self)->U where T: Copy+Into<U>, U: Copy+ From<T>+Mul<Output=U>{
       // self.l * self.b

       let l1 = U::from(self.l);
       let b1 = U::from(self.b);
       l1 * b1
    }
} // the whole code is genered by the compiler 

// l,b are i16 --> i32 
// l,b are i32 --> i64 

// l,b are f32 --> f64
// l,b are f64 -> f128

// Create a UnitStruct may be Circle 
// Use Generics to perform area, perimeter and also +, - +=


