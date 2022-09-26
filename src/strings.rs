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

    // Capacity in bytes
    println!("Capacity: {}", new_string.capacity());

    // Check if empty
    println!("Is Empty: {}", new_string.is_empty());

    // Contains
    println!("Contains 'are': {}", new_string.contains("are"));

    // Replace
    print!("Replace: {}\n", new_string.replace("Hi", "Yes"));

    // Loop through string by whitespace
    for word in new_string.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity
    let mut str = String::with_capacity(10);
    str.push('a');
    str.push('n');
    println!("{}",str);

    // Assertion testing
    assert_eq!(2, str.len()); // checks if str is length of 2, nothing happens cuz assertion passed
    assert_eq!(5, str.len()); // prints an error cuz assertion failed
    assert_eq!(10, str.capacity());

    println!("{}", new_string);

}