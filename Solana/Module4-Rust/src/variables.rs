pub fn variables_example() {
    // declare a variable with value 1
    let mut a = 1;
    println!("a = {}", a);

    //Integers
    let myValue1 = 17;
    let myValue2 : u32 = 28; //Explicitly specify the data type

    //Floating point numbers
    let float_value1 = 28.9;
    let float_value2 : f64 = 3.412;

    //Boolean
    let bool_val = true;
    let float_value3 = false;

    //Characters
    let char1 = 'a';
    let char2: char = '💐';


    // change the value of variable x
    a = 2;    
    println!("a = {}", a);
    println!("myValue1 = {}", myValue1);
    println!("myValue2 = {}", myValue2);
    println!("fl1 = {}", float_value1);
    println!("fl2 = {}", float_value2);    
    println!("bool_val = {}", bool_val);
    println!("fl3 = {}", float_value3);
    println!("char1 = {}", char1);
    println!("char2 = {}", char2);

    //Tuples
    let tup = (4,5,6);

    //Accessing Tuple
    let (x,y,z) = tup; //x=4, y=5, z=6

    let first_element = tup.0;
    let second_element = tup.1;
    let third_element = tup.2;

    //Arrays
    let arr = [6,7,9,10,11];

    let arr1 = [1.2, 3.3, 4.2];

    //Accessing array elements
    let first_element_array = arr[0];
    let second_element_array = arr1[0];
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    println!("first_element: {}", first_element);
    println!("second_element: {}", second_element);
    println!("third_element: {}", third_element);

    println!("first_element_array: {}", first_element_array);
    println!("second_element_array: {}", second_element_array);
}