// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct Numbers(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    // Construct a person
    fn new(first: &str, last: &str) -> Person{
        Person { first_name: first.to_string(), last_name: last.to_string() }
    }

    // Get full name
    fn full_name(&self) -> String{ // self is like .this (refrencing struct of Person)
        format!("{} {}", self.first_name, self.last_name) // similair to println except it doesnt print (no semi colon cuz returning it)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String){ // returns tuple of strings
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };
    c.red = 200; 
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut x = Numbers(21, 22, 23);
    x.2 = 24;
    println!("Numbers: {} {} {}", x.0, x.1, x.2);

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());

    let mut newP = Person::new("Mary", "Doe");
    println!("Person {}", newP.full_name());
    newP.set_last_name("Williams");
    println!("Person {}", newP.full_name());

    println!("Person Tuple {:?}", p.to_tuple());

}