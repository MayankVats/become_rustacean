// Enums define a type which tells us that a value can be one of these.
// The values of enums are called "variants"
// Defining an enum:
enum IpType {
  V4,
  V6,
}

// We can store data in enums.
enum IpTypeV2 {
  V4(String),
  V6(String),
}

enum IpTypeV3 {
  V4(u8, u8, u8, u8),
  V6(String ),
}

fn enums() {
  // Creating instance of IpType's variants
  let four = IpType::V4;
  let six = IpType::V6;

  let home = IpTypeV2::V4(String::from("127.0.0.1"));
  let google = IpTypeV2::V6(String::from("142.250.192.164"));

  let home = IpTypeV3::V4(127, 0, 0, 1);
  let google = IpTypeV3::V6(String::from("142.250.192.164"));
}

// The OPTION enum
// =================
// It is the Rust's standard library's built in generic enum
// It is used to represent a value that can be something or nothing.

// enum Option<T> {
//   Some(T),
//   None
// }

// The RESULT enum
// =================
// It is the Rust's standard library's built in generic enum
// It returns a value that has the possibility of failing

// enum Result<T, E> {
//   Ok(T),
//   Err(E)
// }

// We can apply unwrap() method on OPTION/RESULT which in case of None or Err panic!.



