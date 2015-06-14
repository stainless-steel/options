extern crate options;

use options::Options;

fn main() {
    let mut options = Options::new();

    options.set("foo", 42)
           .set("bar", "To be or not to be?")
           .set("baz", "Hello, world!".to_string());

    println!("foo = {}", options.get::<i32>("foo").unwrap());
    println!("bar = {}", options.get::<&str>("bar").unwrap());
    println!("baz = {}", options.get::<String>("baz").unwrap());
}
