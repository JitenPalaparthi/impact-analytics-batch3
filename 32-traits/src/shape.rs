pub mod shapes {
    pub trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    pub fn Greet(){
     println!("Hello Impact Analytics!");
    }

    pub trait What{
        fn what(&self)->String{
            return "Shape".to_string();
        }
    }

}
