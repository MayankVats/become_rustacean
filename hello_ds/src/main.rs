// Gerneric Data Types
// They are used to create generalised definitions of functions, structs and enums.
// Types of Generic structs or enums can be inferred at the time of instantiation.

// A generic struct
// Its type is defined at compile time
struct Point<T> {
    x: T,
    y: T
}

// Multiple Generic type parameter
struct PointV2<T, U> {
    x: T,
    y: U
}

fn main() {
    let integer = Point { x: 10, y: 20 };
    let float = Point { x: 10.01, y: 20.02 };

    let integer_and_float = PointV2 { x: 10, y: 10.02 };
}

// Generic function
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}