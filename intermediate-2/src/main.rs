use std::env;

use chrono::{Utc, Local};
// use chrono::prelude::*;

use dotenv::dotenv;
use std::ops::Add;

fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let utc = Utc::now();
    let utc2 = Local::now();
    println!("{}", utc);
    println!("{}", utc2);

    let env_var = env::var("ENV");
    match env_var {
        Ok(value) => println!("ENV = {}", value),
        Err(_) => println!("Error reading ENV variable"),
    }

    // Read second environment variable
    let db_var = env::var("DATABASE_URL");
    match db_var {
        Ok(value) => println!("DATABASE_URL = {}", value),
        Err(_) => println!("Error reading DATABASE_URL variable"),
    }

    println!("{}", sum(1, 2));

    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 10.0, height: 4.0 };

    println!("Circle area = {}", c.area());
    println!("Rectangle area = {}", r.area());

    print_area(Circle {radius: 40.00});
}


fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    return  a+b;
}


struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait Area {
    fn area(&self) -> f64;
}

// imp trait
impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

// imp trait
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// T -- generic params
// T: area -- trait bound
fn print_area<T: Area>(shape: T) {
    println!("Area from trait bounds = {}", shape.area());
}