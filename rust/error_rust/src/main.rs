// 可恢复的错误
// 不可恢复的错误

fn main() {
    println!("Hello, world!");
    // thread 'main' panicked at 'crash and burn',
    // panic!("crash and burn");
    let v = vec![1,2,3];
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
    // v[99];


    // Result
    use std::fs::File;
    let f = File::open("hello.txt");
    // thread 'main' panicked at 'Error opening file Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:?}", error)
    //     },
    // };

    use std::io::ErrorKind;
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file {:?}", e),
    //         },
    //         other_errors => panic!("Error opening file {:?}", other_errors),
    //     },
    // };

    // 上段代码等价于以下部分
    // unwrap_or_else
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Error creating file {:?}", error);
    //         })
    //     } else {
    //         panic!("Error opening file {:?}", error);
    //     }
    // });

    // unwrap
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
    // let f = File::open("hello.txt").unwrap();

    // expect
    // thread 'main' panicked at '无法打开文件: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
    // let f = File::open("hello.txt").expect("无法打开文件");

    // 传播错误
    let result = read_username_from_file();
    let result = read_username_from_file_quesmark();


    // panic!
    use std::cmp::Ordering;
    use rand::Rng; // trait
    println!("Guessing game! (1~100)");
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Cannot read line");
    
        // shadow guess
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };

        let guess = Guess::new(guess);
    
        println!("{}", guess.value);
        match guess.value.cmp(&secret_number) {
            // arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            
        }
    }
    
}

// windows cmd:
// SET RUST_BACKTRACE=1 && cargo run

// windows powershell
// $env:RUST_BACKTRACE = '1' | cargo run

// SET RUST_BACKTRACE=1 && cargo run --release

// 传播错误
use std::fs::File;
use std::io;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 如果Result是Ok, Ok中的值就是表达式的结果，并继续执行程序
// 如果Result是Err，Err就是**整个函数**的返回值，如同使用了return
fn read_username_from_file_quesmark() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    // let mut f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    Ok(s)
}

fn read_username_from_file_quesmark_chain() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


// // error[E0308]: mismatched types
// use std::error::Error;
// // ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found `()`
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(());
// }


pub struct Guess{
    value: i32,
}

impl Guess {
    pub fn new(value:i32) -> Guess {
        if value < 1 || value > 100{
            panic!("<1 and >100, now: {}", value);
        }
        Guess {value}
    }

    pub fn value(&self) -> i32{
        self.value
    }
}