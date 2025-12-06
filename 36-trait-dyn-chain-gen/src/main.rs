use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};
fn main() {
    let result = Data::<f32>::new(100.0).add(10.5).add(10.5).sub(10.5).mul(2.0).div(3.0).add(10.0).sub(10.5).get();
    println!("{}",result);

    let result = Data::<i32>::new(100).add(10).add(10).sub(10).mul(2).div(3).add(10).sub(10).get();
    println!("{}",result);
}

// Chain of actions , fluent API, Builder Patten

trait Calc<T> 
where T:Copy+AddAssign+SubAssign+MulAssign+DivAssign{
    fn add(&mut self,d:T)->&mut dyn Calc<T>;
    fn sub(&mut self,d:T)->&mut dyn Calc<T>;
    fn mul(&mut self,d:T)->&mut dyn Calc<T>;
    fn div(&mut self,d:T)->&mut dyn Calc<T>;
    fn get(&self)->T;
}
struct Data<T>{
    data:T,
}

impl<T> Data<T> {
    fn new(d:T)->Data<T>{
        Data { data: d }
    }
}

impl<T> Calc<T> for Data<T>
where T:Copy+AddAssign+SubAssign+MulAssign+DivAssign
{
     fn add(&mut self,d:T)->&mut dyn Calc<T>{
        self.data+=d;
        return self;
     }
     fn sub(&mut self,d:T)->&mut dyn Calc<T>{
        self.data-=d;
        return self;
     }
     fn mul(&mut self,d:T)->&mut dyn Calc<T>{
        self.data*=d;
        return self;
     }
     fn div(&mut self,d:T)->&mut dyn Calc<T>{
        self.data/=d;
        return self;
     }
     fn get(&self)->T {
         self.data
     }
}

