fn dereferencing(reference: &u32) {
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
}

fn destructuring(reference: &u32) {
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
}
fn main() {
    let reference = &4;
    dereferencing(reference);
    destructuring(reference);

    // What if you don't start with a reference ? reference was a '&'
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.

    let _not_reference = 3;

    // Rust provides 'ref' for exactly this purpose. It modifies the
    // assignement so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references can be retrieved
    // via 'ref' and 'ref mut'

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to value: {:?}", r),
    }

    // Use 'ref mut' similarly
    match mut_value {
        mut m => {
            m += 15;
            println!("Added 15 but this is not a reference {:?}", m);
            println!("Direct access is: {:?}", mut_value);
        }
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m);
            println!("Direct access is : {:?}", mut_value);
        }
    }
}
