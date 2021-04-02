enum WebEvent {
    PageLoad,
    KeyPress(char),
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
    }
}
#[derive(Debug)]
enum VeryVersobeEnumOfThingsToDoWithNumbers {
    Add,
}
type Operations = VeryVersobeEnumOfThingsToDoWithNumbers;

fn main() {
    let x = Operations::Add;

    println!("{:?}", x);
}
