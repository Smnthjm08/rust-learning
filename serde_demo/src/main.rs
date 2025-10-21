// on cmd + click on serialize then its taking me to trait why??
// this is importing the trait not the macro
use serde::{Deserialize, Serialize};

// on cmd + click on serialize then its taking me to macro why??
// invoking macros that generate trait impls
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

fn main() {
    let u = User {
        username: String::from("alice"),
        password: String::from("secret"),
    };

    let serialized_string = serde_json::to_string(&u);
    match serialized_string {
        Ok(json) => println!("Serialized User: {}", json),
        Err(e) => eprintln!("Serialization error: {}", e),
    }
}
