fn declare_first() {
    let shadow_bindings;

    {
        shadow_bindings = 2;
        println!("What the f*** are you doing : {}", shadow_bindings);
    }

    println!("a binding: {}", shadow_bindings);

    let another_binding;

    // println!("another binding: {}", another_binding);

    another_binding = "";

    println!("another binding: {}", another_binding);
}

#[allow(dead_code)]
fn shadow_bindings() {
    let shadow_bindings = 1;
    {
        println!("before being shadowed: {}", shadow_bindings);

        let shadow_bindings = "abc";

        println!("shadowed in inner block: {}", shadow_bindings);
    }

    println!("outside inner block: {}", shadow_bindings);
}
fn main() {
    // let long_lived_binding = 1;
    // {
    //     let short_lived_binding = 2;

    //     println!("inner short: {}", short_lived_binding);
    // }

    // // println!("outer short: {}", short_lived_binding);

    // println!("outer long: {}", long_lived_binding);

    // shadow_bindings();
    declare_first();
}
