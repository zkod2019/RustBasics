/*
Primitive Types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (num of bits they take in memory) (u is unsigned so no negative. usually i32 is used)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples (basically lists)
Arrays (fixed length, while vectors r mutable)
*/

pub fn run(){
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.6;

    // Add explicit type
    let z: i64 = 4545454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean can be set in two ways+
    let is_active = true;
    let another_active: bool = true;

    // Get boolean from expression
    let is_greater = 10 > 5;
    let is_lesser:bool = 10 < 5;

    // Chars only allow 1 thing (can be an emoji)
    let a1= 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, is_lesser, a1, face));

}