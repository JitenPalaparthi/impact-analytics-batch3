fn main() {
    let p1 = Person {
        id: 101,
        name: "Jiten".to_string(),
        email: "Jitenp@Outlook.Com".to_string(),
        address: "Trv".to_string(),
    };

    println!("{:#?}", p1);

    let name = "Jiten".to_string();

    let s1 = Student {
        id: 101,
        name: &name,
        email: "Jitenp@Outlook.Com", // &str
        address: "Trv",
    };

    println!("{:#?}", s1);

    // s1 and name dropped

    let str1: String = "hello world".to_string();

    let str2: &String = &str1;

    

}

#[derive(Debug)]
struct Person {
    // if there are no references in struct , we called as struct contsins owned data
    id: i32,
    name: String,
    email: String,
    address: String,
}
// it strores in stack , but some data is in heap

#[derive(Debug)]
struct Student<'a> {
    // if there are no references in struct , we called as struct contsins owned data
    id: i32,
    name: &'a String,
    email: &'a str,
    address: &'a str,
}

trait Address<'a> {
    fn get_city_state(&self) -> (&'a str, &'a str);
}

// trait Address1{
//      fn get_city_state(&self)->String;
// }

// impl<'a> Student<'a>{
//  fn get_city_state(&self)->(&'a str,&'a str){
//     let address_parts:Vec<&str> = self.address.split(',').collect();
//     let city_1 = address_parts[0];
//     let state_2= address_parts[1];
//     return (city_1,state_2)
//  }
// }
// 

impl<'a> Address<'a> for Student<'a> {
    fn get_city_state(&self) -> (&'a str, &'a str) {
        let address_parts: Vec<&str> = self.address.split(',').collect();
        let city_1 = address_parts[0];
        let state_2 = address_parts[1];
        return (city_1, state_2);
    }
}

// in the address of Student --> "city:Bangalore,state:karnataka"
// create a method , parse the address and return city and state as two separate strings

// fn get_city_state()->(&str,&str)

fn get_day(n: u8) -> &'static str {
    match n {
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        7 => "Saturday",
        _ => "No day",
    }
}
