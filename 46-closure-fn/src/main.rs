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

}

fn add(i: i32, j: i32) -> i32 {
    return i + j;
}

fn calc1(a: i32, b: i32, fn1: fn(i: i32, j: i32) -> i32) -> i32 {
    return fn1(a, b);
}

fn calc2(a: i32, b: i32, fn1:&dyn Fn(i32,i32) -> i32) -> i32 {
    return fn1(a, b);
}

// fn --> function pointer
// Fn --> trait
