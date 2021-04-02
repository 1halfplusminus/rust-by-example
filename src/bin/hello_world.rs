use std::fmt;
// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, 'a', true);

    println!("long tuple first value: {}", long_tuple.0);

    println!("{:?}", long_tuple);

    let tuple_of_tuples = ((1u8, 'a', "test"), (1, 2, 3));

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    reserse((1, true));
}

fn reserse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}
