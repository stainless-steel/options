extern crate options;

use options::Options;

#[test]
fn workflow() {
    let mut options = Options::new();

    options.set("a", 42)
           .set("b", true)
           .set("c", "Hi, there!")
           .set("d", String::from("Hello, world!"));

    macro_rules! check(
        ($options:expr, $name:expr, $value:expr, $kind:ty) => (
            assert_eq!($options.get::<$kind>($name).unwrap(), $value)
        );
    );

    check!(options, "a", &42, i32);
    check!(options, "b", &true, bool);
    check!(options, "c", &"Hi, there!", &str);
    check!(options, "d", "Hello, world!", String);
}
