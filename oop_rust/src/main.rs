mod rectangle;
use rectangle::Rectangle;

// We need to bring trait in scope to use the methods.
use rectangle::ShapeMethods;

fn main() {
    // if we want to use write method we need to make it mutable
    let mut rectangle = Rectangle {
        height: 20,
        width: 30
    };

    println!("Area is: {}", rectangle.area());
    println!("Perimeter is: {}", rectangle.perimeter());

    rectangle.set_height(40);
    println!("Area is: {}", rectangle.area());
    println!("Perimeter is: {}", rectangle.perimeter());


    rectangle.set_width(3);
    println!("Area is: {}", rectangle.area());
    println!("Perimeter is: {}", rectangle.perimeter());
}
