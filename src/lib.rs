//! Data structure for managing named parameters.

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
    pub fn get<T: Any>(&self, name: &str) -> Option<&T> {
        self.map.get(name).and_then(|ref value| value.downcast_ref::<T>())
    }

    /// Set the value of a parameter.
    #[inline]
    pub fn set<'l, T: Any>(&'l mut self, name: &str, value: T) -> &'l mut Options {
        self.map.insert(name.to_string(), Box::new(value));
        self
    }
}
