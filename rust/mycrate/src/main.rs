// https://crates.io/

// use mycrate::kinds::PrimaryColor;
// use mycrate::utils::mix;
// - 使用 pub use: 
use mycrate::PrimaryColor;
use mycrate::mix;
fn main() {
    println!("Hello, world!");

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
