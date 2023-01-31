/*
Primitive Types--
Intergers: u8,i8,u16...
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

/*
Rust is a statically typed language, which means that it must know the types of all
variables at compile time, howeber, the compiler can usually inder what type we want to used
based on the value and how we use it.
*/

pub fn run(){
    // Default is i32
    let _x = 1;

    // Default is "f64"
    let _y = 2.5;

    // Add explicit type
    let _z: i64 = 4545445454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let _a1 = 'a';
    let _face = '\u{1F600}';

    println!("{:?}", (_x, _y, _z, is_active, is_greater, _a1, _face));
}