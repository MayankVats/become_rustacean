
// Any function in rust is defined with "fn" keyword
fn main() {
    // function call
    let result = greet_and_add(1, 2);
    println!("result: {}", result);

    let (x, y) = give_tuple();
    println!("Tuple({}, {})", x, y);
}


// By convention function name in rust follow snake_case.
// return type of the function is defined by -> 
// We can use return statement or leave the last line without semicolon which should evaluate to some expression.
fn greet_and_add(x: i32, y: i8) -> i32 {
    println!("Hello, world!"); 
    x + y as i32
}

// functions can also return multiple values with tuple.
// if a function returns nothing then it returns an empty tuple () called unit.
fn give_tuple() -> (i32, i32) {
    (1, 2)
}