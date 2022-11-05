# Options [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a data structure for managing named parameters.

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

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/stainless-steel/options/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/options/actions/workflows/build.yml
[documentation-img]: https://docs.rs/options/badge.svg
[documentation-url]: https://docs.rs/options
[package-img]: https://img.shields.io/crates/v/options.svg
[package-url]: https://crates.io/crates/options
