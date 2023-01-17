// struct is a collection of related fields.
// It is a blueprint for the compiler on how to layout fields in memory.
// Its data is on stack.
// Defining a struct:
struct User {
  active: bool,
  // String is a struct
  username: String,
  email: String,
  sign_in_count: u64,
}

// Tuple like struct
struct Point(i32, i32);

fn structures() {
  // Instantiating a struct
  // Its fields are laid out side by side in memory.
  let admin_user = User {
      active: false,
      // "String" is also a struct, which is on stack.
      // The function call below creates a String struct.
      // This is a static method (method that belongs to type itself)
      // static methods are accessed using ::
      username: String::from("admin"),
      // This creates memory on the heap
      // and the reference to that memory is stored in "String" struct.
      email: String::from("admin@gmail.com"),
      sign_in_count: 1
  };

  // This is an instance method
  // instance methods are accessed using dot operator.
  println!("Email length: {}", admin_user.email.len());

  // We can Instantiating struct with another struct
  let super_admin = User {
      active: true,
      ..admin_user
  };

  let locationA = Point(110, 234);
}

// A function can also return a struct
fn build_user(email: String, username: String) -> User {
  User {
      email,
      username,
      active: true,
      sign_in_count: 1
  }
}

/*
Memories In Rust
================
1. Data Memory
  - It is fixed in size and static.
  - Static means always available through lifetime of the program.
2. Stack Memory
  - For data that is declared as variables in functions.
  - Location of memory never changes for the duration of function call.
3. Heap Memory
  - It is used for data that is created while the application is running.
  - It is dynamic nature and hence slow.
  - Allocation(data added), deallocation(data removed).

*/
