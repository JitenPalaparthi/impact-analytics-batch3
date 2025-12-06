
use crate::shape::shapes::{Shape,What};

pub struct Circle(f32);

impl Circle{
    pub fn new(s:f32)->Self{
        Self(s)
    }
}

impl Shape for Circle{
    fn area(&self) -> f64 {
        (3.14 * (self.0 * self.0)) as f64
    }
     fn perimeter(&self) -> f64 {
        ( 3.14 * self.0  * self.0) as f64
    }
}

impl What for Circle{} // This is kind of a validater 
// either really implemet What trait by implementing a fn what
// or you can just satisfy the trait called What but dont implement the method