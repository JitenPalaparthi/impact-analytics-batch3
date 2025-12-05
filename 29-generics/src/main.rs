use std::ops::{Mul,Add};
fn main() {
   let r1 = Rect::new(12,14);

   let a1 = r1.area();

   println!("area:{}",a1);


let r1 = Rect::new(12.23,14.234);

   let a1 = r1.area();

   println!("area:{}",a1);


   let r1 = Rect::new(12.23f32,14.234f32);

   let a1 = r1.area();

   println!("area:{}",a1);

   let r1 = Rect::new(12.23f32,14.234f32);
   let r2 = Rect::new(22.23f32,54.234f32);

   let r3 = r1+r2;

   println!("{:#?}",r3);

   let a1 = r3.area();

   println!("area:{}",a1);

   let r1 = Rect::<i32>::new(12, 13);

  let a1 = r1.area();

   println!("area:{}",a1);


}

#[derive(Debug,Clone)]
struct Rect<T> {
    l: T,
    b: T,
}

impl<T> Rect<T> where T:Mul<Output=T>+Copy {
    fn new(l: T, b: T) -> Self {
        return Rect { l: l, b: b };
    }
    fn area(&self)->T{
        self.l * self.b
    }
}

impl<T> Add for Rect<T> where T:Add<Output=T>{
    type Output = Rect<T>; // asserted type
    fn add(self, rhs: Self) -> Self::Output {
        Rect {
            l: self.l + rhs.l,
            b: self.b + rhs.b,
        }
    }
}
