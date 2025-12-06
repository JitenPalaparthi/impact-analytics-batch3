use std::{collections::HashMap, ops::Add};

fn main() {
    
    let x = 100; 

    let y = x; // this is based on trait .. Copy

    let mut z = x + y; // Add 

    z+=1; // AddAssign

    let k:i64 = z as i64; // traint --> From 


    let i1:Integer = Integer(100);

    let i2 = i1;

    println!("{:?}",i1);

    //Copy

    // Copy,Clone, Debug,Display, Eq, PartialEq, Drop , Into, From, Add, AddAssign,

    let r1= get_sum_g1(i1,i2);

    let r2 = get_sum_g1(123.12f32, 13.43f32);


    let r3= get_sum_g2(mybool(true), mybool(false));

} 

// What is a trait --> It is similar to interface + default implementation
// trait is a part of the core language.
// trait can be used as bounds 



#[derive(Debug)]
struct Integer(i32);


impl std::ops::Add for Integer{
     type Output = Self;
     fn add(self, rhs: Self) -> Self::Output{
        Self(self.0+rhs.0)
     }
}

#[derive(Debug,Copy,Clone)]
struct mybool(bool);

impl std::ops::Add for mybool{
     type Output = Self;
     fn add(self, rhs: Self) -> Self::Output{
        if self.0==true && rhs.0 == true{
            return Self(true);
        }else{
            Self(false)
        }
     }
}


impl Copy for Integer{}

impl Clone for Integer{

fn clone(&self) -> Self {       
    *self
    } 
}

// Generics are code generator --> Compile time
// Monomorphization vs Type Eraser

fn get_sum(a:i32,b:i32)->i32{
    return a+b
}

fn get_sum_g1<T:Add<Output=T>+Copy>(a:T,b:T)->T{
    return a + b
}

fn get_sum_g2<T>(a:T,b:T)->T where T:Add<Output=T>+Copy{
    return a + b
}
// T can be of anything




// create your own type.. myfloat32(f32)
// create your own type.. myfloat64(f64)

// Implement type casting between myfloat32 and myfloat64

// Implement From and Into traits 
