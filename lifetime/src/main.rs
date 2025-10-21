// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = String::from("xyz");

//     let result = longest_string(&string1, &string2);
//     println!("The longest string is {}", result);
// }

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest_string(&string1, &string2);
    println!("The longest string is {}", result);
}

// lifetime param 'a
fn longest_string<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


// fn longest_string<'a, 'b>(s1: &'a String, s2: &'b String) -> &'a String {
//     if s1.len() > s2.len() {
//         return &s1;
//     } else {
//         return &s2;
//     }
// }

// fn longest_string(s1: String, s2: String) -> String {
//     if s1.len() > s2.len() {
//         return s1;
//     } else {
//         return s2;
//     }
// }
