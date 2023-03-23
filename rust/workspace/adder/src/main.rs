// cargo run -p adder


use add_one;
fn main() {
    println!("Hello, world!");

    let num = 10;
    println!(
        "{}+1={}",
        num,
        add_one::addone(num)
    )
}
