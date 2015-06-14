# Options [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a data structure for managing named parameters.

## [Documentation][doc]

## Example

```rust
use options::Options;

let mut options = Options::new();

options.set("foo", 42)
       .set("bar", "To be or not to be?")
       .set("baz", "Hello, world!".to_string());

println!("foo = {}", options.get::<i32>("foo").unwrap());
println!("bar = {}", options.get::<&str>("bar").unwrap());
println!("baz = {}", options.get::<String>("baz").unwrap());
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: https://img.shields.io/crates/v/options.svg
[version-url]: https://crates.io/crates/options
[status-img]: https://travis-ci.org/stainless-steel/options.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/options
[doc]: https://stainless-steel.github.io/options
