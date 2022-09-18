// Primitive str = Immutabel fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let hello = "Hello"; // primitive
    let mut new_string = String::from("Hi"); // growable

    // Get length
    println!("Length: {}", new_string.len());

    // Push a char
    new_string.push('.'); // only works for chars and only if the string is mut
    
    // Push str
    new_string.push_str(" How are you?");
    println!("{}", new_string);

}