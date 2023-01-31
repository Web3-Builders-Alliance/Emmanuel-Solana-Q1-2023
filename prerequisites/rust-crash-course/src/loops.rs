// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite Loop
    /* loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    } */

    // While Loop (FizzBuzz)
    /* while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        }
        else if count % 3 == 0 {
            println!("fizz");
        }
        else if count % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", count);
        }

        //Increment
        count += 1;
    } */

    // For range
    for _x in 0..100 {
        if _x % 15 == 0 {
            println!("fizzbuzz");
        }
        else if _x % 3 == 0 {
            println!("fizz");
        }
        else if _x % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", _x);
        }

    }
}