fn main() {
    let a = 7;
    let mut b = a; // here the value of a is copied and assigned to b
    b = b + 1;
    println!("a = {}, b = {}", a, b);

    // same thing above with strings
    // Strings are stored in Heap and its reference is stored on stack.
    let s1 = String::from("hello");

    // here if s2 and s1 both point to same memory location in heap
    // then, when they go out of scope rust will try to free memory twice
    // which will result in memory corruption called "double free error".
    // To avoid this scenario, rust makes s1 invalid by "move".
    // Moving the ownership of data to s2. Hence there can be only 1 owner at a time
    let s2 = s1;
    println!("s1 = {}, s2 = {}", s1, s2);

    // the code above for strings won't compile
    // Rust compiler does not know:
    // 1. Do we want two copies of the string?
    // 2. Do we want one copy and two variables pointing to it?

    // Case 1: Copy of the value
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Case 2: One copy and two variables pointing to it.
    // we can have as many (read-only)refrences as we want as shown below.
    // You can only use original when the borrowed item is dropped.
    let s2 = &s1; // it is "borrowing" with the intention to read the value
    println!("s1 = {}, s2 = {}", s1, s2);

    // Case: Changing the value, that the variable has a reference to.
    // Mutable reference(borrow)
    let mut s1 = String::from("hello");
    let s2 = &mut s1; // it is "borrowing" with intention to change the value
    s2.push('!');
    println!("s2 = {}", s2);
    println!("s1 = {}", s1);
}

// This function takes ownership of the value passed in variable
// and also returns the original value to be used later after the function call.
fn calculate_length(s: String) -> (usize, String) {
    (s.len(), s)
}

// This function instead takes a reference to the variable passed as parameter.
// Reference basically stores a pointer pointing to the referred value.
fn calc_len(s: &String) -> usize {
    s.len()
}

// String in rust are made up of 3 parts:
// 1. ptr -> a pointer that points to a location in memory(heap) where the contents of string are stored.
// 2. len -> the length of the string in bytes.
// 3. capacity -> the memory in bytes recieved, by string, by the allocator.

// Function parameters also follow the concept of ownership as same as the variables above follow.
// Every time we pass a value in the function call, it is moved to the parameter of the function call and if we want to use that value after function call we won't be able to.
// In order to use that value after the function call we need to return that value from the function.