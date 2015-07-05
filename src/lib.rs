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

/// A collection of named parameters.
pub struct Options {
    map: HashMap<Name, Value>,
}

/// A parameter name.
pub type Name = String;

/// A parameter value.
pub type Value = Box<Any>;

/// An iterator over names and values.
pub struct Pairs<'l> {
    iterator: hash_map::Iter<'l, Name, Value>,
}

/// An iterator over names and mutable values.
pub struct PairsMut<'l> {
    iterator: hash_map::IterMut<'l, Name, Value>,
}

/// An iterator over names.
pub struct Names<'l> {
    iterator: iter::Map<hash_map::Iter<'l, Name, Value>, fn((&'l Name, &'l Value)) -> &'l Name>,
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

    /// Return an iterator over names and values.
    pub fn iter<'l>(&'l self) -> Pairs<'l> {
        Pairs { iterator: self.map.iter() }
    }

    /// Return an iterator over names and mutable values.
    pub fn iter_mut<'l>(&'l mut self) -> PairsMut<'l> {
        PairsMut { iterator: self.map.iter_mut() }
    }

    /// Return an iterator over names.
    #[inline]
    pub fn names<'l>(&'l self) -> Names<'l> {
        fn first<'l>((name, _): (&'l Name, &'l Value)) -> &'l Name { name }
        Names { iterator: self.map.iter().map(first) }
    }
}

impl<'l> IntoIterator for &'l Options {
    type Item = (&'l Name, &'l Value);
    type IntoIter = Pairs<'l>;

    #[inline]
    fn into_iter(self) -> Pairs<'l> {
        self.iter()
    }
}

impl<'l> IntoIterator for &'l mut Options {
    type Item = (&'l Name, &'l mut Value);
    type IntoIter = PairsMut<'l>;

    #[inline]
    fn into_iter(self) -> PairsMut<'l> {
        self.iter_mut()
    }
}

impl<'l> Iterator for Pairs<'l> {
    type Item = (&'l Name, &'l Value);

    #[inline]
    fn next(&mut self) -> Option<(&'l Name, &'l Value)> {
        self.iterator.next()
    }
}

impl<'l> Iterator for PairsMut<'l> {
    type Item = (&'l Name, &'l mut Value);

    #[inline]
    fn next(&mut self) -> Option<(&'l Name, &'l mut Value)> {
        self.iterator.next()
    }
}

impl<'l> Iterator for Names<'l> {
    type Item = &'l Name;

    #[inline]
    fn next(&mut self) -> Option<&'l Name> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use Options;

    #[test]
    fn get() {
        let options = setup();

        macro_rules! test(
            ($name:expr, $value:expr, $kind:ty) => (
                assert_eq!(options.get::<$kind>($name).unwrap(), $value)
            );
        );

        test!("a", 42, i32);
        test!("b", true, bool);
        test!("c", "Hi, there!", &str);
        test!("d", "Hello, world!".to_string(), String);
    }

    #[test]
    fn get_ref() {
        let options = setup();

        macro_rules! test(
            ($name:expr, $value:expr, $kind:ty) => (
                assert_eq!(options.get_ref::<$kind>($name).unwrap(), $value)
            );
        );

        test!("a", &42, i32);
        test!("b", &true, bool);
        test!("c", &"Hi, there!", &str);
        test!("d", "Hello, world!", String);
    }

    #[test]
    fn get_mut() {
        let mut options = setup();

        macro_rules! test(
            ($name:expr, $value:expr, $kind:ty) => ({
                *options.get_mut::<$kind>($name).unwrap() = $value;
                assert_eq!(options.get::<$kind>($name).unwrap(), $value);
            });
        );

        test!("a", 24, i32);
        test!("b", false, bool);
        test!("c", "Hi, here!", &str);
        test!("d", "Bye, world!".to_string(), String);
    }

    #[test]
    fn iter_mut() {
        let mut options = setup();
        for (_, value) in &mut options {
            *value = Box::new(69);
        }

        macro_rules! test(
            ($name:expr) => (assert_eq!(options.get_ref::<i32>($name).unwrap(), &69));
        );

        test!("a");
        test!("b");
        test!("c");
        test!("d");
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
