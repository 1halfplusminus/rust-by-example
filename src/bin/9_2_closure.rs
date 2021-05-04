fn main() {
    // Increment via function
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annoted = |i: i32| i + 1;
    let closure_infered = |i| i + 1;

    let i = 1;

    println!("function: {}", function(i));
    println!("closure annoted: {}", closure_annoted(i));
    println!("closure infered: {}", closure_infered(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}
