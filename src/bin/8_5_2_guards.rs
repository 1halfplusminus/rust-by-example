fn match_pair(position: (i32, i32)) {
    match position {
        (x, y) if x == 800 => println!("x hit the border of screen, y: {}", y),
        (x, y) if y == 600 => println!("y hit the border of screen, x: {}", x),
        (x, y) if x < 800 && y < 600 => println!("In screen x: {},y: {}", x, y),
        (x, y) if x >= 800 || y >= 600 => println!("Outside of the screen"),
        (x, y) => println!("I don't know where you are x: {}, y: {}", x, y),
    }
}
fn main() {
    match_pair((50, 50));
    match_pair((800, 10));
    match_pair((10, 600));
    match_pair((1000, 10));
    match_pair((1000, 2000));
}
