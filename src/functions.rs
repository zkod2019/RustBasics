// Functions - Used to store blocks of code for re-use

pub fn run(){
    greeting("Hello", "John");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure (can use outside variables)
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32{ // the arrow shows what the function will return
    n1 + n2 // no semicolon = what the funtion will return
}