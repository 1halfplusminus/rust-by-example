use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Number {
        Number { value }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        return self.value;
    }
}
fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    let int = 8;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
