//! Data structure for managing named parameters.
//!
//! ## Example
//!
//! ```
//! use options::Options;
//!
//! let mut options = Options::new();
//!
//! options.set("foo", 42)
//!        .set("bar", "To be or not to be?")
//!        .set("baz", "Hello, world!".to_string());
//!
//! println!("foo = {}", options.get::<i32>("foo").unwrap());
//! println!("bar = {}", options.get::<&str>("bar").unwrap());
//! println!("baz = {}", options.get::<String>("baz").unwrap());
//! ```

use std::any::Any;
use std::collections::HashMap;

/// A collection of named parameters.
pub struct Options {
    map: HashMap<String, Box<Any>>,
}

impl Options {
    /// Create a new collection of named parameters.
    #[inline]
    pub fn new() -> Options {
        Options { map: HashMap::new() }
    }

    /// Get the value of a parameter.
    #[inline]
    pub fn get<T: Any + Clone>(&self, name: &str) -> Option<T> {
        self.map.get(name).and_then(|ref value| value.downcast_ref::<T>())
                          .and_then(|value| Some(value.clone()))
    }

    /// Get the value of a parameter by reference.
    #[inline]
    pub fn get_ref<T: Any>(&self, name: &str) -> Option<&T> {
        self.map.get(name).and_then(|ref value| value.downcast_ref::<T>())
    }

    /// Set the value of a parameter.
    #[inline]
    pub fn set<'l, T: Any>(&'l mut self, name: &str, value: T) -> &'l mut Options {
        self.map.insert(name.to_string(), Box::new(value));
        self
    }
}

#[cfg(test)]
mod tests {
    use Options;

    #[test]
    fn get() {
        macro_rules! test(
            ($options:expr, $name:expr, $value:expr, $kind:ty) => (
                assert_eq!($options.get::<$kind>($name).unwrap(), $value)
            );
        );

        let options = setup();
        test!(options, "a", 42, i32);
        test!(options, "b", true, bool);
        test!(options, "c", "Hi, there!", &str);
        test!(options, "d", "Hello, world!".to_string(), String);
    }

    #[test]
    fn get_ref() {
        macro_rules! test(
            ($options:expr, $name:expr, $value:expr, $kind:ty) => (
                assert_eq!($options.get_ref::<$kind>($name).unwrap(), $value)
            );
        );

        let options = setup();
        test!(options, "a", &42, i32);
        test!(options, "b", &true, bool);
        test!(options, "c", &"Hi, there!", &str);
        test!(options, "d", "Hello, world!", String);
    }

    fn setup() -> Options {
        let mut options = Options::new();

        options.set("a", 42)
               .set("b", true)
               .set("c", "Hi, there!")
               .set("d", "Hello, world!".to_string());

        options
    }
}
