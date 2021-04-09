enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    RBG(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
}

fn match_color(color: Color) {
    println!("What color is it?");

    match color {
        Color::Yellow => println!("The color is yellow"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
        Color::Red => println!("The color is red"),
        Color::RBG(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSL(hue, saturation, lightness) => println!(
            "hue: {}, saturation: {}, lightness: {}",
            hue, saturation, lightness
        ),
        Color::CMY(cyan, magenta, yellow) => {
            println!("cyan: {}, magenta: {}, yellow: {}", cyan, magenta, yellow)
        }
        Color::HSV(hue, saturation, volume) => println!(
            "hue: {}, saturation: {}, volume: {}",
            hue, saturation, volume
        ),
        /* _ => println!("It's not a color"), */
    }
}
fn main() {
    match_color(Color::Red);
    match_color(Color::Blue);
    match_color(Color::Yellow);
    match_color(Color::Green);
    match_color(Color::HSL(1, 10, 100));
    match_color(Color::CMY(50, 30, 10));
    match_color(Color::HSV(5, 9, 10));
    match_color(Color::RBG(100, 100, 100))
}
