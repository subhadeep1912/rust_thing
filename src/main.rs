mod lib;
// use crate::lib;


// fn larget_val<T>(list: &[T]) -> T 
// {
//     let mut largest:T = list[0];
//     for &i in list {
//         if i > largest { largest = i}
//     }
//     largest
// }
enum FanState<T>
{
    On(T),
    Off
}



struct Person {
    name: String,
    age: u8,
    height: u16
}

impl Person {
    pub fn bark(&self) -> String
    {
        String::from("Samuel L Jackson yelling")
    }

    pub fn likes(&self, person:&Person) -> bool
    {
        let temp_age:i16 = self.age.into();
        let temp_per:i16 = person.age.into();
        let diff:i16= temp_age - temp_per;
        let diff = diff.abs();
        if diff < 3{ true }
        else { false }
    } 
}



fn main() {
    // println!("Hello, world!");
    let avg_per = Person {
        name: String::from("Mr Normal"),
        age: 34,
        height: 130
    };

    let f:FanState<String> = FanState::On(String::from("bruh"));
    match f {
        FanState::On(String) => println!("bruh"),
        FanState::Off => println!("save energy"),
        _ => ()
    }

    let ant_per = Person {
        name: String::from("Mrs Bruh"),
        age: 32,
        height: 180
    };

    let like:bool = avg_per.likes(&ant_per);

    let ye = String::clone(&avg_per.name);
    let bye:String =  String::clone(&ant_per.name);
    let res = avg_per.bark();
    println!("{ye}: {res}");
    if like {println!("{ye} likes {bye}")}
    else {println!("nope")}

    lib::temp();
}
