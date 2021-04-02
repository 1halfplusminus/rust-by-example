type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64_t;

    println!(
        "{} + {} inches = {} units",
        nanoseconds,
        inches,
        nanoseconds + inches
    )
}
