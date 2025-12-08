 mod shapes;
 use crate::shapes::rectangle::rect::Rect;
 use crate::shapes::greet::Greet as g1;
 
// use crate::shapes::shape::Shape;

fn main() {

    use crate::shapes::rectangle::greet::Greet as g2;

    let r1 = Rect::new(12.34,34.34);
    println!("area:{:.2}",r1.area());
    println!("perimeter:{:.2}",r1.perimeter());
    g1();
    g2();
}

// binary crate 
// 