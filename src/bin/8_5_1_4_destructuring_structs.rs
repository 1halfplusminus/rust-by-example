struct FooBar<'a> {
    foo: (u32, u32),
    bar: &'a str,
}

fn match_struct(structure: FooBar) {
    match structure {
        FooBar { foo: (1, b), bar } => println!("foo.a is 1, foo.b = {}, bar = {}", &b, bar),
        FooBar { foo: (a, 2), bar } => println!("foo.b is 2, foo.a = {}, bar ={} ", a, bar),
        FooBar { bar, .. } => println!("bar = {}, we don't care about foo", bar),
    }
}
fn main() {
    match_struct(FooBar {
        bar: "test",
        foo: (10, 10),
    });

    match_struct(FooBar {
        bar: "test2",
        foo: (1, 10),
    });

    match_struct(FooBar {
        bar: "test3",
        foo: (10, 2),
    });
}
