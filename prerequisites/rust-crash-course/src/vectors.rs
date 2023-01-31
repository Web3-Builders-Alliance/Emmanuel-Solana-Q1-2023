// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run(){
    //let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    // Add on vector
    numbers.push(5);
    numbers.push(6);

    // Pop off the last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}