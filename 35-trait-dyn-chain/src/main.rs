fn main() {
   

    let result = Data::new(100.0).add(10.5).add(10.5).sub(10.5).mul(2.0).div(3.0).add(10.0).sub(10.5).get();

    println!("{}",result);
}

// Chain of actions , fluent API, Builder Patten

trait Calc{
    fn add(&mut self,d:f32)->&mut dyn Calc;
    fn sub(&mut self,d:f32)->&mut dyn Calc;
    fn mul(&mut self,d:f32)->&mut dyn Calc;
    fn div(&mut self,d:f32)->&mut dyn Calc;
    fn get(&self)->f32;
}
struct Data{
    data:f32,
}

impl Data {
    fn new(d:f32)->Data{
        Data { data: d }
    }
}

impl Calc for Data{
     fn add(&mut self,d:f32)->&mut dyn Calc{
        self.data+=d;
        return self;
     }
     fn sub(&mut self,d:f32)->&mut dyn Calc{
        self.data-=d;
        return self;
     }
     fn mul(&mut self,d:f32)->&mut dyn Calc{
        self.data*=d;
        return self;
     }
     fn div(&mut self,d:f32)->&mut dyn Calc{
        self.data/=d;
        return self;
     }
     fn get(&self)->f32 {
         self.data
     }
}

