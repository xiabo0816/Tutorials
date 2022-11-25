use std::io; // prelude
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("Guessing game! (1~100)");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("{}", secret_number);

    // let mut foo: i32 = 1;
    // let bar = foo;
    // foo = 2;

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Cannot read line");
    
        // shadow guess
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };
    
        println!("{}", guess);
        match guess.cmp(&secret_number) {
            // arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            
        }
    }


    println!("Press any key to exit");
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {},
        Err(_) => {},
    };
    

}
