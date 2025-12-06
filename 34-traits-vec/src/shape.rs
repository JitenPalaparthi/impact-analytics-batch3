pub mod shapes {
    pub trait Shape : What{
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    pub fn Greet(){
     println!("Hello Impact Analytics!");
    }

    pub trait What{
        fn what(&self)->String{ // default implementation
            return "Shape".to_string();
        }
    }

}
