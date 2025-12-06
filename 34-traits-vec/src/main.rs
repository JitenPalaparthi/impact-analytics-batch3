mod rect;
mod shape;
mod square;
mod circle;

use std::collections::btree_map::Range;

use crate::rect::Rect;
use crate::circle::Circle;
use crate::shape::shapes::{Shape};
use crate::square::Square;
use tinyrand::{Rand, RandRange, StdRand};
fn main() {
    let mut rand = StdRand::default();
  
    
   
   let mut shape_vec:Vec<Box<dyn Shape>> = Vec::<Box<dyn Shape>>::new();

   // any type that implements the Shape can be inserted into vector

   shape_vec.push(Box::new(Rect::new(12.23f32,14.34f32)));
   shape_vec.push(Box::new(Circle::new(12.23f32)));
   shape_vec.push(Box::new(Square::new(12.23f32)));
   shape_vec.push(Box::new(Rect::new(13.23f32,14.34f32)));
   shape_vec.push(Box::new(Circle::new(13.23f32)));
   shape_vec.push(Box::new(Square::new(1.23f32)));
   shape_vec.push(Box::new(Rect::new(14.23f32,14.34f32)));
   shape_vec.push(Box::new(Circle::new(14.23f32)));
   shape_vec.push(Box::new(Square::new(14.23f32)));

    let num = rand.next_lim_u32(8);
  //let r =  get_rang(shape_vec.len());

    let s= &shape_vec[num as usize];
    s.area();
    s.perimeter();

    

   for s in &shape_vec{
    //  println!("Area of {}:{:.2}",s.what(), s.area());
    //  println!("perimeter of {} :{:.2}",s.what(),s.perimeter());
     Shape(s.as_ref())
   }
   
}




fn Shape(s: &dyn Shape) {  // dynamic dispatch, the compiler would not generate the code 
    // this is how you can call the object s
   println!("Area of {}:{:.2}",s.what(), s.area());
   println!("perimeter of {} :{:.2}",s.what(),s.perimeter())
}
