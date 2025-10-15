use std::fs;

fn main() {
    // string object stored on the heap.
    // 's1' owns the data "sumanth".
    let s1: String = String::from("sumanth");

    // an immutable reference (&) to s1.
    // does not take ownership — s1 still owns the data.
    let s2: &String = &s1;

    // other immutable reference to the same String.
    // multiple immutable references are allowed because they don't modify data.
    let s3: &String = &s1;

    // Yet another immutable reference.
    let s4: &String = &s1;

    // Print all four variables.
    // s1 is still valid because ownership was never moved.
    // s2, s3, s4 just "borrow" the data temporarily.
    println!("{}, {}, {}, {}", s1, s2, s3, s4);

    let mut str: String = String::from("string ");

    let ref1 = &mut str;
    ref1.push_str("string");
    let ref2 = &str;
    println!("{}", ref2);

    let r = Rectangle {
        width: 10.0,
        height: 10.0,
    };

    println!("{}, {}", r.height, r.width);

    // enum and pattern matching
    let circle = Shape::Circle(10.0);
    let square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);

    println!("Circle area: {}", calculate_area(circle));
    println!("Square area: {}", calculate_area(square));
    println!("Rectangle area: {}", calculate_area(rectangle));

    main_greet();
}

struct Rectangle {
    height: f32,
    width: f32,
}

enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32),
}

fn calculate_area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
        Shape::Square(len) => len * len,
        Shape::Rectangle(len, wid) => len * wid,
    }
}

fn main_greet() {
    let greeting_file_result: Result<String, std::io::Error> = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}
