// Constants
// They are always mutable
// They should be type annotated
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn data_types() {
    // Data types (Scalar)
    // ===================
    // i8, i16, i32, i64, i128, isize (signed integer)
    // u8, u16, u32, u64, u128, usize (unsigned integer)
    // f32, f64 (floating point number)
    // char (single character) (4 bytes in size to represent Unicode Scalar Value)
    // str (text with a length known at runtime, "string slice")
    // bool (true/false)

    // Variables in Rust are created using "let" keyword
    // assigning a value to the variable is called "binding"
    // By default the variables are immutable.
    let x: u32 = 102;

    // This is a mutable variable, its value can be changed later.
    // Declaring a variable with same name again is called "shadowing"
    let mut x: u32;
    x = 10;

    // "{}" the curly brackets acts as placeholder for the values to print.
    println!("Value of x is: {}", x);

    // Data types (Compound)
    // =====================
    // tuple:
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);
    // tuple destructuring:
    let (x, y, z) = my_tuple;
    // accessing tuple's values:
    println!("{} {} {}", my_tuple.0, my_tuple.1, my_tuple.2);
    // A tuple without any value is called a unit.

    // arrays:
    // Declaring an array containing 5 elements of type i32
    let my_array: [i32; 5];
    // Defining an array containing 10 elements of same value 5.
    let init_array = [5; 10];
    //
    let quarter_one = ["Jan", "Feb", "Mar"];
}