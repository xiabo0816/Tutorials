const MAX_NUMBER: i32 = 1;

fn main(){
    println!("const: {}", MAX_NUMBER);
    let x = 4;
    println!("x: {}", x);

    // shadowing
    let x = x + 1;
    println!("x: {}", x);

    // shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // error[E0282]: type annotations needed
    // let guess = "42".parse().expect("asdf");
    let guess: u32 = "42".parse().expect("asdf");
    println!("guess:{}", guess);

    // float, default f64
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x:{}", x);
    println!("y:{}", y);

    // bool, one bit
    let t = true;
    let f: bool = false;
    println!("t:{}", t);
    println!("f:{}", f);

    // char, 4 bytes, U+0000-U+D7FF, U+E000-U+10FFFF, 
    let x = 'z';
    let y: char = 'Ͷ';
    let z = '☺';
    println!("x:{}", x);
    println!("y:{}", y);
    println!("z:{}", z);

    // tuple, 
    let tup: (i32, f64, u8) = (400, 5.3, 1);
    println!("tup.0: {}", tup.0);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // array
    let a = [1, 2, 3, 4, 5];
    println!("a[0]: {}", a[0]);
    let a = [3;5];
    println!("a[4]: {}", a[4]);
    // error: this operation will panic at runtime, index out of bounds:
    // println!("a[6]: {}", a[6]);
    let a:[u32; 5] = [1, 2, 3, 4, 5];
    println!("a[0]: {}", a[0]);

    // Vector

    // function
    let arguments = 5;
    another_function(arguments);
    
    // function default return value
    let x = five();
    println!("fn five: {}", x);

    // if
    let number = 6;
    if number < 5 {
        println!("if case, true");
    }else{
        println!("if case, false");
    }

    // if let
    // error[E0308]: `if` and `else` have incompatible types
    // let number = if true {5} else {6.0};
    let number = if true {5} else {6};
    println!("if let: {}", number);

    if number % 4 == 0 {
        println!("if case, divisible by 4");
    }else if number % 3 == 0 {
        println!("if case, divisible by 3");
    }else if number % 2 == 0 {
        println!("if case, divisible by 2");
    }else{
        println!("if case, cannot divisible by 4,3,2");
    }

    // match
    match number % 4 {
        0 => println!("match, divisible 4 remain 0"),
        1 => println!("match, divisible 4 remain 1"),
        2 => println!("match, divisible 4 remain 2"),
        _ => {
            println!("match, cannot divisible by 4,3,2");
        }
    }

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop: {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("while: {}", number);
        number -= 1;
    }

    // for
    let a = [1,2,3,4,5];
    for e in a.iter(){
        println!("for: {}", e);
    }

    // range, rev
    for e in (1..4).rev() {
        println!("range, rev: {}", e);
    }
}

// function, snake case
fn another_function(parameters:i32){
    println!("another_function: {}", parameters);
    // statement
    // error: expected expression, found statement (`let`)
    // let x = (let y = 1);

    // expression1: {5 + 6}
    let x = {5 + 6};
    println!("another_function x: {}", x);

    /* expression2: {
         let x = 5 + 6;
         x + 3
    }*/
    let x = {
        let x = 5 + 6;
        // x + 3; //()
        x + 3
    };
    println!("another_function x: {}", x);
}

// 函数默认返回值是，最后一个表达式(expression)的值
fn five() -> i32 {
    5
}