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
use std::collections::hash_map::{self, HashMap};
use std::iter;

type Key = String;
type Value = Box<Any>;

/// A collection of named parameters.
pub struct Options {
    map: HashMap<Key, Value>,
}

/// An iterator over the names of parameters.
pub struct Names<'l> {
    iterator: iter::Map<hash_map::Iter<'l, Key, Value>, fn((&'l Key, &'l Value)) -> &'l str>,
}

impl Options {
    /// Create a collection of named parameters.
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

    /// Get a reference to the value of a parameter.
    #[inline]
    pub fn get_ref<T: Any>(&self, name: &str) -> Option<&T> {
        self.map.get(name).and_then(|value| value.downcast_ref::<T>())
    }

    /// Get a mutable reference to the value of a parameter.
    #[inline]
    pub fn get_mut<T: Any>(&mut self, name: &str) -> Option<&mut T> {
        self.map.get_mut(name).and_then(|value| value.downcast_mut::<T>())
    }

    /// Set the value of a parameter.
    #[inline]
    pub fn set<'l, T: Any>(&'l mut self, name: &str, value: T) -> &'l mut Options {
        self.map.insert(name.to_string(), Box::new(value));
        self
    }

    /// Return an iterator over the names of the stored parameters.
    #[inline]
    pub fn names<'l>(&'l self) -> Names<'l> {
        fn first<'l>((name, _): (&'l Key, &'l Value)) -> &'l str { name }
        Names { iterator: self.map.iter().map(first) }
    }
}

impl<'l> Iterator for Names<'l> {
    type Item = &'l str;

    #[inline]
    fn next(&mut self) -> Option<&'l str> {
        self.iterator.next()
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

    #[test]
    fn get_mut() {
        macro_rules! test(
            ($options:expr, $name:expr, $value:expr, $kind:ty) => ({
                *$options.get_mut::<$kind>($name).unwrap() = $value;
                assert_eq!($options.get::<$kind>($name).unwrap(), $value);
            });
        );

        let mut options = setup();
        test!(options, "a", 24, i32);
        test!(options, "b", false, bool);
        test!(options, "c", "Hi, here!", &str);
        test!(options, "d", "Bye, world!".to_string(), String);
    }

    #[test]
    fn names() {
        let options = setup();
        let mut names = options.names().collect::<Vec<_>>();
        names.sort();
        assert_eq!(names, &["a", "b", "c", "d"]);
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
