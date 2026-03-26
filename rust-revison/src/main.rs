// 1. conditional loops
// fn main() {
//     let is_male = false;
//     let mut is_above_18 = true;

//     if is_male {
//         println!("you are a male");
//     } else if !is_male {
//         println!("{}", is_above_18);
//         println!("you are a female")
//     }
//     is_above_18 = true;

//     if is_male && is_above_18 {
//         println!("you are a legal male")
//     } else if !is_male && is_above_18 {
//         println!("you are a legal female")
//     }
// }

// 2. Strings in rust
// fn main(){
//     let greet = String::from("hello world!");
//     println!("{}", greet)
// }

// 3. conditional and loops
// fn get_first_word(sentence: String) -> String {
//     let mut first_word = String::from("");

//     for c in sentence.chars() {
//         first_word.push(c);
//         if c == ' ' {
//             break;
//         }
//     }

//     first_word
// }

// fn main() {
//     let sentence = String::from("hello hello");

//     let first_word = get_first_word(sentence);

//     println!("{}", first_word);
// }

// 4. ownership
// fn main() {
//     let x = 1; // crated on stack
//     {
//         let y = 3; // created on stack
//     }

//     println!("{}", y); // throws error
// }

// fn main() {
//     let s1 = String::from("value");
//     let s2 = s1;
//     // println!("{}", s1);
//     // error borrow of moved value: `s1`

//     println!("{}", s2);
// }

// 5. reference and borrowing
// fn main() {
//     let s1 = String::from("hello");

//     let s2 = &s1; // reference is given to the string
//     let s3 = &s1; // reference is given to the string

//     println!("s1 {}", s1);
//     println!("s2 {}", s2);
//     println!("s3 {}", s3);
// }

// 6. struct and impl
// struct Person{
//     name: String,
//     age: i32
// }

// impl Person{
//     fn greet(&self){
//      println!("{}, {}", self.age , self.name)
//     }
// }

// impl Person {
//     fn current_age(&self) -> i32 {
//         return self.age
//     }
// }

// fn main() {
//     let p = Person {
//         name: String::from("Alice"),
//         age: 25,
//     };

//     p.greet();
//    let u2 = p.current_age();
//    println!("{}", u2)
// }

// 7. enum and pattern matching
// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     let ans = match shape {
//         Shape::Circle(radius) => 3.14 * radius * radius,
//         Shape::Rectangle(width, height) => width * height,
//         Shape::Square(side) => side * side,
//     };

//     return ans;
// }

// fn main() {
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);

//     let test = String::from("dsfdf");

//     for (index, char) in test.chars().enumerate() {}
//     println!("area of circle or raidus 2: {}", calculate_area(circle));
//     println!("area of square or raidus 2: {}", calculate_area(square));
//     println!(
//         "area of rectangle or raidus 2: {}",
//         calculate_area(rectangle)
//     );
// }

// 8. error handling
// enum Result<T,E> {
//    Ok(T),
//    Err(E)
// }

// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Err(String::from("Cannot divide by zero"))
//     } else {
//         Ok(a / b)
//     }
// }

// optional enum
// trait

// fn main(){
//     println!("hello world!")
// }

// macros - run at compile time
// println!("Hello")
// vec![1,2,3]
// format!("hi {}", name)

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    pwd: String,
}

fn main() {
    let u = User {
        name: "wew".to_string(),
        pwd: "wew".to_string(),
        age: 23,
    };
    println!("{:?}", u.name);
    println!("{:?}", u.age);
    println!("{:?}", u.pwd);
}
