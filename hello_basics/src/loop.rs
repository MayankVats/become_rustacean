
// In rust there are 3 kinds of loops:
// loop, while and for
fn loops() {
  let mut x: i32 = 0;
  
  // this loop will run endlessly until break is called.
  // we can also return value from the loop
  let sum = loop {
    println!("Hello");
    x += 1;
    if x == 10 {
      break x + 2;
    }
  };

  println!("Sum: {}", sum);

  // The for..in loop iterates over values from any expression that evaluates to iterator.
  // An "iterator" is an object. We can ask it: "What is the next item you have?"

  for x in 0..5 {
    println!(":{}:", x);
  }
}