use std::arch::aarch64::int32x2_t;

fn main() {
    let r = calc1(10, 20, add);

    println!("add->r {}", r);
    let sub = |i: i32, j: i32| -> i32 {
        return i - j;
    };

    let r = calc1(10, 20, sub);
    println!("add->r {}", r);

    let r = calc1(10, 20, |i: i32, j: i32| -> i32 {
        return i * j;
    });

    let r = calc2(10, 20, &add);

    println!("add->r {}", r);

    let r = calc2(10, 20, &sub);
    println!("add->r {}", r);

    let r = calc2(10, 20, &|i: i32, j: i32| -> i32 {
        return i * j;
    });

    let mut vec1 = Vec::<&dyn Fn(i32, i32) -> i32>::new();

    vec1.push(&|i: i32, j: i32| -> i32 {
        return i * j;
    });

    vec1.push(&|i: i32, j: i32| -> i32 {
        return i * j;
    });

    vec1.push(&add);

    vec1.push(&sub);


    let (a,b)=(100,200);
    for f in vec1{
        let r = f(a,b);
        println!("result:{}",r)
    }

}

fn add(i: i32, j: i32) -> i32 {
    return i + j;
}

fn calc1(a: i32, b: i32, fn1: fn(i: i32, j: i32) -> i32) -> i32 {
    return fn1(a, b);
}

fn calc2(a: i32, b: i32, fn1: &dyn Fn(i32, i32) -> i32) -> i32 {
    return fn1(a, b);
}

fn calc3(a: i32, b: i32, c: i32, fn1: &dyn Fn(i32, i32, i32) -> i32) -> i32 {
    return fn1(a, b, c);
}

// fn --> function pointer
// Fn --> trait

// functions and closures can be treted as varialbe


// take a hashmap <String,Fn(i32,i32)->i32>
// add closures with the keys add,sub, mul, div 
// execute them in a loop
// get a specific funtion using get and execute