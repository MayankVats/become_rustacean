// A macro is defined by following syntax:
// macro_rules! <macro_name_here> {
//     () => {
        
//     };
// }

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// In the macro below "expr" is a designator (metavariable) for expressions
// The arguments of macro are prefixed by $ sign and annotated by designator as shown below
// $name_of_variable:meta_variable
macro_rules! x_and_y {
    (x => $e:expr) => (println!("X: {}", $e));
    (y => $e:expr) => (println!("Y: {}", $e));
}

// This macro creates a function with the name provided
// In the macro below "ident" is a metavariable for identifiers.
macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! logic {
    ($l:expr; and $r:expr) => {
        println!("{:?} and {:?} is {:?}", stringify!($l), stringify!($r), $l && $r)
    };

    ($l:expr; or $r:expr) => {
        println!("{:?} or {:?} is {:?}", stringify!($l), stringify!($r), $l || $r)
    };
}

macro_rules! compr {
    ($id1: ident | $id2: ident <- [$start: expr; $end: expr], $cond: expr) => {
        {
            let mut vec = Vec::new();

            for num in $start..$end + 1 {
                if $cond(num) {
                    vec.push(num);
                }
            }

            vec
        }
    };
}

fn even(x: i32) -> bool {
    x%2 == 0
}

fn odd(x: i32) -> bool {
    x%2 != 0
}

fn main() {
    say_hello!();

    x_and_y!(x => 10);
    x_and_y!(y => 10 + 20);

    build_fn!(hello);
    hello();

    logic!(1 == 1; and 2 != 1 + 1);

    let evens = compr!(x | x <- [1;10], even);
    println!("{:?}", evens);

    let odds = compr!(x | x <- [1;10], odd);
    println!("{:?}", odds);
}
