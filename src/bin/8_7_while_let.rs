fn awkward_while_match() {
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            // Quit te loop when the destructure fails:
            _ => {
                break;
            } // Why shoud this be required? there must be a better way!
        }
    }
}
fn cleaner_while_let() {
    let mut optional = Some(0);
    // This reads: "while `let` destructure `` optional"
    // into `Some(i)`, evaluate the block '({})'. Else ``break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
fn main() {
    awkward_while_match();
    cleaner_while_let();
}
