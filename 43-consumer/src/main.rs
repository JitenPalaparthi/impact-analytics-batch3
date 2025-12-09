use package_management::shapes::rectangle::rect::Rect;
use package_management::shapes::greet::Greet as g1;
 
 use package_management::shapes::shape::shape::Shape;

fn main() {
    use package_management::shapes::rectangle::greet::Greet as g2;
    let r1 = Rect::new(12.34,34.34);
     println!("area:{:.2}",r1.area());
     println!("perimeter:{:.2}",r1.perimeter());
    g1();
    g2();
}

// binary crate 
// 