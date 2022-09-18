pub fn run(){
    // Print to Console
    println!("Hi from the print.rs file");

    // Formatting
    println!("Num: {}", 42);
    println!("{} is from {}", "Jane", "Spain");

    // Positional Arguments
    println!("{0} is from {1}, and {0} likes to {2}", "Jane", "Spain", "eat");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Jane", activity ="video games");

    // Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug Traits
    println!("{:?}", (12, true, "hello") ); // tuple

    // Basic Math
    println!("10 + 10 = {}", 10 + 10)
}