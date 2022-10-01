// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // let numbers: [data type: length of array]. if you remove one num in the array it wont work cuz it expects 5 numbers

    // Re-assign value
    numbers[2] = 20;

    // Print all values
    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2]; // printing the first two values from the array
    println!("Slice: {:?}", slice);
}