// Loops - Used to iterate until a condition is met

pub fn run(){
    let mut count = 0;
    let mut fizzbuzz = 0;
    
    // Infinite Loop
    loop{
        count += 1;
        println!("Number {}", count);

        if count == 10{
            break;
        }
    }

    // While Loop (FizzBuzz if div by 3 or 5 print FizzBuzz) 
    while fizzbuzz <= 100{
        if fizzbuzz % 15 == 0 {
            println!("FizzBuzz");
        } else if fizzbuzz % 3 == 0{
            println!("Fizz");
        } else if fizzbuzz % 5 == 0{
            println!("Buzz");
        }else{
            println!("{}", fizzbuzz);
        }
        
        // Increment
        fizzbuzz += 1;
    }

    // For Range
    for x in 0..20{
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0{
            println!("Fizz");
        } else if x % 5 == 0{
            println!("Buzz");
        }else{
            println!("{}", x);
        }
    }


}