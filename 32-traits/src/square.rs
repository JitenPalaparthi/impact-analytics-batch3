
use crate::shape::shapes::{Shape,What};

pub struct Square(f32);

impl Square{
    pub fn new(s:f32)->Self{
        Self(s)
    }
}

impl Shape for Square{
    fn area(&self) -> f64 {
        (self.0 * self.0) as f64
    }
     fn perimeter(&self) -> f64 {
        (2.0 * (self.0 + self.0)) as f64
    }
}


impl What for Square{
    fn what(&self)->String{
        "Square".to_string()
    }
}