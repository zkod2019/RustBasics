// Tuples group together values of different types
// Max 12 elements

pub fn run(){
    let person: (&str, &str, i8) = ("John", "Spain", 37); // adding types
    print!("{} is from {} and is {}", person.0, person.1, person.2);
}