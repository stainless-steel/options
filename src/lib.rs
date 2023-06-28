//! Data structure for managing named parameters.
//!
//! ## Example
//!
//! ```
//! use options::Options;
//!
//! let mut options = Options::new();
//!
//! options
//!     .set("foo", 42)
//!     .set("bar", "To be or not to be?")
//!     .set("baz", "Hello, world!".to_string());
//!
//! println!("foo = {}", options.get::<i32>("foo").unwrap());
//! println!("bar = {}", options.get::<&str>("bar").unwrap());
//! println!("baz = {}", options.get::<String>("baz").unwrap());
//! ```

use std::any::Any;
use std::collections::hash_map::{self, HashMap};

/// A collection of named parameters.
#[derive(Debug)]
pub struct Options(HashMap<Name, Value>);

/// A parameter name.
pub type Name = String;

/// A parameter value.
#[derive(Debug)]
pub struct Value(Box<dyn Any>);

/// An iterator over parameters.
pub struct Parameters<'l> {
    iterator: hash_map::Iter<'l, Name, Value>,
}

/// An iterator over mutable parameters.
pub struct ParametersMut<'l> {
    iterator: hash_map::IterMut<'l, Name, Value>,
}

/// An iterator over names.
pub struct Names<'l> {
    #[allow(clippy::type_complexity)]
    iterator:
        std::iter::Map<hash_map::Iter<'l, Name, Value>, fn((&'l Name, &'l Value)) -> &'l Name>,
}

impl Options {
    /// Create a collection of named parameters.
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Options {
        Options(HashMap::new())
    }

    /// Get the value of a parameter.
    #[inline]
    pub fn get<T: Any + Clone>(&self, name: &str) -> Option<T> {
        self.0.get(name).and_then(|value| value.get())
    }

    /// Get a reference to the value of a parameter.
    #[inline]
    pub fn get_ref<T: Any>(&self, name: &str) -> Option<&T> {
        self.0.get(name).and_then(|value| value.get_ref())
    }

    /// Get a mutable reference to the value of a parameter.
    #[inline]
    pub fn get_mut<T: Any>(&mut self, name: &str) -> Option<&mut T> {
        self.0.get_mut(name).and_then(|value| value.get_mut())
    }

    /// Set the value of a parameter.
    #[inline]
    pub fn set<'l, T: Any>(&'l mut self, name: &str, value: T) -> &'l mut Options {
        self.0.insert(name.to_string(), Value(Box::new(value)));
        self
    }

    /// Check the presence of a parameter.
    #[inline]
    pub fn has(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Return an iterator over parameters.
    pub fn iter(&self) -> Parameters<'_> {
        Parameters {
            iterator: self.0.iter(),
        }
    }

    /// Return an iterator over mutable parameters.
    pub fn iter_mut(&mut self) -> ParametersMut<'_> {
        ParametersMut {
            iterator: self.0.iter_mut(),
        }
    }

    /// Return an iterator over names.
    #[inline]
    pub fn names(&self) -> Names<'_> {
        fn first<'l>((name, _): (&'l Name, &'l Value)) -> &'l Name {
            name
        }
        Names {
            iterator: self.0.iter().map(first),
        }
    }
}

impl Value {
    /// Get the value.
    #[inline]
    pub fn get<T: Any + Clone>(&self) -> Option<T> {
        self.0.downcast_ref::<T>().cloned()
    }

    /// Get a reference to the value.
    #[inline]
    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    /// Get a mutable reference to the value.
    #[inline]
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }

    /// Set the value.
    #[inline]
    pub fn set<T: Any>(&mut self, value: T) {
        self.0 = Box::new(value);
    }
}

impl<'l> IntoIterator for &'l Options {
    type Item = (&'l Name, &'l Value);
    type IntoIter = Parameters<'l>;

    #[inline]
    fn into_iter(self) -> Parameters<'l> {
        self.iter()
    }
}

impl<'l> IntoIterator for &'l mut Options {
    type Item = (&'l Name, &'l mut Value);
    type IntoIter = ParametersMut<'l>;

    #[inline]
    fn into_iter(self) -> ParametersMut<'l> {
        self.iter_mut()
    }
}

impl<'l> Iterator for Parameters<'l> {
    type Item = (&'l Name, &'l Value);

    #[inline]
    fn next(&mut self) -> Option<(&'l Name, &'l Value)> {
        self.iterator.next()
    }
}

impl<'l> Iterator for ParametersMut<'l> {
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
    use super::Options;

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
        test!("e", vec![4u8, 2u8], Vec<u8>);
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
        test!("e", &vec![4u8, 2u8], Vec<u8>);
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
        test!("e", vec![2u8, 4u8], Vec<u8>);
    }

    #[test]
    fn iter_mut() {
        let mut options = setup();
        for (_, value) in &mut options {
            value.set(69);
        }

        macro_rules! test(
            ($name:expr) => (assert_eq!(options.get_ref::<i32>($name).unwrap(), &69));
        );

        test!("a");
        test!("b");
        test!("c");
        test!("d");
        test!("e");
    }

    #[test]
    fn has() {
        let options = setup();
        assert_eq!(options.has("a"), true);
        assert_eq!(options.has("z"), false);
    }

    #[test]
    fn names() {
        let options = setup();
        let mut names = options.names().collect::<Vec<_>>();
        names.sort();
        assert_eq!(names, &["a", "b", "c", "d", "e"]);
    }

    fn setup() -> Options {
        let mut options = Options::new();

        options
            .set("a", 42)
            .set("b", true)
            .set("c", "Hi, there!")
            .set("d", "Hello, world!".to_string())
            .set("e", vec![4u8, 2u8]);

        options
    }
}
