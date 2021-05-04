fn apply<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}
fn main() {
    use std::mem;

    let greating = "hello";
    // A non copy type.
    // 'to_owned' creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // greating is by reference: requires Fn
        println!("I said {}.", greating);

        // Mutation forces farwell to be captured by mutable reference. Now requires FnMut
        farewell.push_str("!!!");
        println!("Then i screamed {}.", farewell);
        println!("Now i can sleep. zzzzz");

        /* mem::drop(farewell); */
    };

    apply(diary);

    // double statisfies apply_to_3's trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
