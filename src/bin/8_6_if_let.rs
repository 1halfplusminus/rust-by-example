// For some use case, when matching enums, match is awkward example.
fn awkward_match() {
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i)
        }
        _ => {} // Required because match is exhaustive, doesn't it seem
                // like wasted space ?
    }
}
fn cleaner_if_let() {
    let number: Option<i32> = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched: {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched: {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!")
    }
}
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}
fn if_let_enums(level: u32) {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(level);

    if let Foo::Bar = a {
        println!("a is a foobar")
    }
    if let Foo::Baz = b {
        println!("b is a foobaz")
    }
    if let Foo::Qux(i) = c {
        println!("c is a foqux at level {}", i);
    }
    // binding
    if let Foo::Qux(i @ 5) = c {
        println!("Wow level {}, you're high level", i);
    }
}
enum FooNonPartialEq {
    Bar,
    Barz,
}
fn if_let_non_partial_eq() {
    let a = FooNonPartialEq::Bar;

    /*  if Foo::Bar == a {
        // ^-- this causes a compile-time error. Use 'if let' instead
        println!("a is foobar");
    } */
    if let FooNonPartialEq::Bar = a {
        println!("a is foobar");
    }
}
fn main() {
    awkward_match();
    cleaner_if_let();
    if_let_enums(5);
    if_let_non_partial_eq();
}
