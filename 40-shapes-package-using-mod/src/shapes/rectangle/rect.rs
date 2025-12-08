use crate::shapes::shape::Shape;
pub struct Rect{
    pub l:f32,
    pub b:f32,
}

impl Rect{
    pub fn new(l:f32,b:f32)->Self{
        Self{l:l,b:b}
    }
}

impl  Shape for Rect{

    fn area(&self)->f64{
        return (self.l * self.b)as f64;
    }
     fn perimeter(&self)->f64{
        return (2.0 *(self.l + self.b)) as f64;
    }
}

