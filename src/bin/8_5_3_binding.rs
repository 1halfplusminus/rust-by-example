// Indirectly accessing a variable makes it impossible to branch and use that variable
// without rebinding, match provides the @ sigil for binding values to names

fn match_binding(value: u32) {
    match value {
        n @ 0 => println!(
            "I haven't celebrated my first birthday yet, i'm a sanscoeur at : {} ",
            n
        ),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn match_binding_enum(option: Option<u32>) {
    match option {
        Some(n @ 42) => println!("The Answer: {};", n),
        Some(n) => println!("Not interesting ... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }
}

fn main() {
    println!("Tell me what type of person you are");
    match_binding(age());
    match_binding_enum(some_number());
    match_binding_enum(Some(42));
}

fn age() -> u32 {
    11
}

fn some_number() -> Option<u32> {
    Some(32)
}
