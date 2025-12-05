use std::ops::{Add,Sub};

//use std::ops::AddAssign 

//+=
fn main() {

    let mut r1 = Rect::new(12.23,53.34);

    let r2 = Rect::new(32.45,31.46);

    r1 = r1+r2.clone();


    //self, &self -? &mut self


   // r1 = r1.add(r2);

   // let r2 = Rect::new(32.45,31.46);

    let r3 = r1-r2; // operator overloading

    // let a= 10;
    // let b= 20;
    // let mut k = a+b; // arithmetic // i32 has implemented Add

     //k+=1; // AddAssign

  //  let (r1,r2)=(10,12);
   // let r2 = r1+r2; // what is the type at the right side --> Rect Rhs = Self

   //let r3 = r1.add(r2);

 //  let r3 = r1+r2;

  // println!("{:#?}",r1);


   let num1 = Integer32(1232231);
   let num2 = Integer32(4343343);

   let num3 = num1 +num2;

   println!("{:?}",num3);

}

#[derive(Debug,Clone)]
struct Rect{
    l:f32,
    b:f32,
}

impl Rect{
    fn new(l:f32,b:f32)-> Self{
        return Rect{
            l:l,b:b
        }
    }
}

impl Add for Rect{
type Output = Rect; // asserted type
    fn add(self, rhs: Self) -> Self::Output {
        Rect{
            l: self.l + rhs.l,
            b: self.b + rhs.b
        }
    }
}


impl Sub for Rect{
type Output = Rect; // asserted type
    fn sub(self, rhs: Self) -> Self::Output {
        Rect{
            l: self.l - rhs.l,
            b: self.b - rhs.b
        }
    }
}



#[derive(Debug)]
struct Integer32(i32); // Unit struct

#[derive(Debug)]
struct Integer64(i64); // Unit struct

impl Add for Integer32{
    type Output = Integer64;

    fn add(self, rhs: Self) -> Self::Output {
       Integer64((self.0+rhs.0) as i64)
    }
}