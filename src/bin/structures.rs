#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        return self.top_left.x * self.bottom_right.x;
    }
}

fn main() {
    let name = String::from("Peter!");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 1.0, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let point2: Point = Point { y: 0.0, ..point };

    println!("point 2 coordinates: ({}, {})", point2.x, point2.y);

    let rect: Rectangle = Rectangle {
        top_left: point,
        bottom_right: point2,
    };
    println!("Area : {}", rect.area());
}
