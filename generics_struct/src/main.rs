use std::ops::Mul;
fn main() {
   let r1 = Rect::<i32>::new(10, 20);
   let r2= Rect::new(12.34f32,13.45f32);

   let a1 = r1.area();
   println!("{}",a1);
   
   let a1 = r1.area();
   println!("{:.2}",a1);

   // 

}

struct Rect<T>{
    l:T,
    b:T,
}

impl<T> Rect<T>{
    fn new(l:T,b:T)->Self{
        Self { l: l, b: b }
    }
    fn area(&self)->T where T:Mul<Output=T>+Copy{
        self.l * self.b
    }
}

// The implement the T for myfloat(f32)
// what all to implement for myfloat(f32) --> Copy, Clone , Mul
