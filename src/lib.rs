use std::any::Any;
use std::collections::HashMap;

/// A collection of named options.
pub struct Options {
    map: HashMap<String, Box<Any>>,
}

impl Options {
    /// Create a new collection of options.
    #[inline]
    pub fn new() -> Options {
        Options { map: HashMap::new() }
    }

    /// Get the value of an option.
    #[inline]
    pub fn get<T: Any>(&self, name: &str) -> Option<&T> {
        self.map.get(name).and_then(|ref value| value.downcast_ref::<T>())
    }

    /// Set the value of an option.
    #[inline]
    pub fn set<'l, T: Any>(&'l mut self, name: &str, value: T) -> &'l mut Options {
        self.map.insert(name.to_string(), Box::new(value));
        self
    }
}
