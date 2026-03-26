// // fn main() {
// //     println!("Hello, world!");
// // }

// DECLARATIVE MACROS

// macro_rules! say_hello {
//     () => {
//         println!("Hello, world!");
//     };
// }

// fn main() {
//  say_hello!();
//    return;
// }

// COMPLEX MACROS

#[derive(Debug)]

struct User {
    username: String,
    password: String,
    age: u32,
}

fn main() {
    let u = User {
        username: String::from("user1"),
        password: String::from("password123"),
        age: 30,
    };

    println!("username: {:?}", u.username); // read username
    println!("password: {:?}", u.password); // read password
    println!("age: {}", u.age);
    println!("{:?}", u);
    // print!("{}", u); // Display trait
}
