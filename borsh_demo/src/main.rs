use borsh::{BorshDeserialize, BorshSerialize, to_vec, from_slice};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    username: String,
    email: String,
    age: u8,
}

fn main() {
    let user = User {
        username: "alice".to_string(),
        email: "alice@gmail.com".to_string(),
        age: 30u8,
    };

    println!("Before serialization: {:?}", user);

    // serialize
    let bytes = to_vec(&user).unwrap();
    println!("Serialized bytes: {:?}", bytes);

    // deserialize
    let decoded_user: User = from_slice(&bytes).unwrap();
    println!("Decoded user: {:?}", decoded_user);
}
