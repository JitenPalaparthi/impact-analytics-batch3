fn main() {
    
    let s1 = "Sunday".to_string(); // not str and string

    match s1.as_str(){ // s1.as_str() stores in ds at compile
        "Sunday"=> println!("1"),
        _ => println!("Something none")
    }

    let s1 = "Sunday";
    let r =match s1{
        "Sunday"=>1,
        "Monday"=>2,
        _=>0,
    };


    let num = 1 ;

    let day = 2;

    match num{
        day => println!("some day"),
        _ => println!("no day")
    }

}


// &self 
// &mut self


enum direction1{
    east,west,south,north
}

enum direction2{
    east, west, south, north
}