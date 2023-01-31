// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let _arr1 = [1, 2, 3];
    let _arr2 = _arr1;

    //println!("Values: {:?}", (_arr1, _arr2));

    // Vector
    let _vec1 = vec![1, 2, 3];
    let _vec2 = &_vec1;

    println!("Values: {:?}", (&_vec1, _vec2));

}