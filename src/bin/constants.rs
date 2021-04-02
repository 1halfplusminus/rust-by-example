static LANGUAGE: &str = "Rust";
static THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n> THRESHOLD
}

fn main() {
    let n = 18;    

    println!("This is {}", LANGUAGE);
    
    println!("This is a integer {:?}",n);
    
    println!("The threshold is {}", THRESHOLD);
    
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

    
}