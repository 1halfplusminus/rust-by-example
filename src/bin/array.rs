use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
fn main() {
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("First element of the array: {}", xs[0]);

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);

    analyze_slice(&ys[1..4]);
}
