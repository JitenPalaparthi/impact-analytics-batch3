mod shape;
mod rectangle;
mod square;

use crate::rectangle::rect::rect::Rect;
use crate::shape::shape::Shape;
fn main() {
    let r1 = Rect::new(12.34,34.34);
     println!("area:{:.2}",r1.area());
     println!("perimeter:{:.2}",r1.perimeter());
     square::square::Greet();
}
