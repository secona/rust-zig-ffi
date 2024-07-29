#[link(name = "math_zig")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn sub(a: i32, b: i32) -> i32;
    fn mul(a: i32, b: i32) -> i32;
    fn div(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        println!("10 + 2 = {}", add(10, 2));
        println!("10 - 2 = {}", sub(10, 2));
        println!("10 * 2 = {}", mul(10, 2));
        println!("10 / 2 = {}", div(10, 2));

        println!("5 * (2 + 1) = {}", mul(5, add(2, 1)));
    }
}
