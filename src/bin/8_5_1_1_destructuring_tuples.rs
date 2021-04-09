fn match_triple(triple: (i32, i32, i32)) {
    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First value is 0, seconde is {}, and third is {}", y, z),
        (1, ..) => println!("First value is 1 and i don't care about the rest"),
        _ => println!("It doesn't matter what they are"),
    }
}
fn main() {
    match_triple((0, -2, 3));
    match_triple((1, 1, 1));
    match_triple((2, 2, 2));
}
