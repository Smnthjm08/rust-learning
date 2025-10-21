fn main() {
    println!("Hello, world!"); //macro

    let result: i32 = sum(5, 10);
    println!("The sum of 5 and 10 is: {}", result);

    let number = 4;
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    name();
    create_string();
    create_string_two();

    // let name: String = String::from("sumanth JM");
    // let len = get_len(name);
    // println!("{}", name);  // ownership is moved to the get_len() fn

    let name: String = String::from("sumanth ----");
    let (len, name) = get_len(name);
    // println!("{}", len);
    // println!("{}", name); // ownership is moved to the get_len() fn
    println!("{}{}", len, name);
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn is_even(n: i32) -> bool {
    return n % 2 == 0;
}

fn name() {
    let mut name: String = String::from("Alice");
    println!("Hello, {}", name);

    name.push_str("string");
}

fn create_string() {
    let mut name = String::from("Sumanth ");

    name.push_str("JM");
    println!("{}", name);
}

fn create_string_two() {
    let mut name = String::from("Sumanth ");

    name.push_str("JM");
    println!("{}", name);

    let name2: String = name;
    // let _name2: String = name; // used variables can be handled here

    // borrow of moved value: `name`, value borrowed here after move
    // println!("{}", name);

    println!("{}", name2)
}

// fn get_len(s: String) -> usize{
//     return s.len();
// }

fn get_len(s: String) -> (usize, String) {
    return (s.len(), s);
}
