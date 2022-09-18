// Vars hold primitive data or refrences to data
// Vars are immutable by default (by default u cant reassign them)
// Rust is a block-scoped language

pub fn run(){
    let name = "John";
    let age = 21;
    // age = 22; <-- cannot assign twice to immutable variable

    let mut color = "blue";
    println!("My name is {}, I am {}, and my fav color is {}", name, age, color);

    color = "purple";
    println!("My name is {}, I am {}, and my fav color is {}", name, age, color);

    // Define Constant
    const ID: i32 = 001; // u can use const but need to add a type and full caps
    println!("ID: {}", ID);

    // Assign Mult Vars
    let (my_name, my_age) = ("John", 24);
    println!("{} is {}", my_name, my_age);
}