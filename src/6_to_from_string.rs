use std::fmt;

struct Hero {
    name: String,
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I'm {} the hero", self.name)
    }
}
fn main() {
    let hero = Hero {
        name: String::from("John"),
    };
    println!("{}", hero.to_string());
}
