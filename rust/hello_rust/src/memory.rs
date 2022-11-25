// Stack vs Heap

fn main(){
    // scope
    // s不可以用
    let s = "hello";
    println!("scope: {}", s);

    // Stack vs Heap
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("String Heap: {}", s);


    // move stack
    let x = 5;
    let y = x;
    println!("move stack: {}, {}", x, y);

    // move heap
    let s1 = String::from("hello");
    let s2 = s1;
    // error[E0382]: borrow of moved value: `s1`, value borrowed here after move
    // println!("move heap: {}", s1);
    println!("move heap: {}", s2);

    // move > shallow copy
    // clone > deep copy

    // Clone stack
    let x = 5;
    let y = x;
    println!("Clone stack: {}, {}", x, y);
    
    // Clone stack+heap
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("Clone heap: {}, {}", s1, s2);

    // copy trait: u32, f64, bool, char, tuple(i32, i32)
    // drop trait

    
    // makes_copy
    let x = 5;
    makes_copy(x);
    println!("out of makes_copy: {}", x);

    // take_ownership
    let s = String::from("hello");
    // - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    take_ownership(s);
    // - value moved here
    // error[E0382]: borrow of moved value: `s`, ^ value borrowed here after move
    // println!("out of take_ownership: {}", s);

    // gives_ownership
    let s1 = gives_ownership();
    println!("gives_ownership: {}", s1);

    // takes_and_gives_back
    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);
    // error[E0382]: borrow of moved value: `s1`, ^^ value borrowed here after move
    // println!("takes_and_gives_back: {}", s1);
    println!("takes_and_gives_back: {}", s2);


    // use and return
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    // error[E0382]: borrow of moved value: `s1`, ^^ value borrowed here after move
    // println!("use and return: {}", s1);
    println!("calculate_length: {}, {}", s2, len);

    // reference
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("calculate_length_ref: {}, {}", s1, len);


    // reference mut
    let mut s1 = String::from("hello");
    let len = calculate_length_ref_mut(&mut s1);
    println!("calculate_length_ref: {}, {}", s1, len);
    
    // ref mut once
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // error[E0499]: cannot borrow `s1` as mutable more than once at a time
    // ^^^^^^^ second mutable borrow occurs here
    // let s3 = &mut s1;
    println!("ref mut once: {}", s2);

    // ref mut once, in different scope
    let mut s1 = String::from("hello");
    {
        let s3 = &mut s1;
        println!("ref mut once, in different scope: {}", s3);
    }
    let s2 = &mut s1;
    println!("ref mut once, in different scope: {}", s2);

    // reference with static
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    println!("reference with static: {}, {}", s2, s3);

    // reference mut togather with static
    // let mut s1 = String::from("hello");
    // let s2 = &s1;
    // let s3 = &mut s1;
    // error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
    // ^^^^^^^ mutable borrow occurs here
    // println!("reference mut togather with static: {}, {}", s1, s2);

    // dangling pointer
    // let r = dangle();


    // slice
    let mut s = String::from("hello");
    let word_index = first_word(&s);
    s.clear();
    println!("slice: {}, {}", word_index, s.len());

    // slice, must in utf8 string
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..s.len()];
    println!("slice: {}, {}", hello, world);

    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];
    println!("slice: {}, {}, {}", hello, world, whole);


    // slice
    let mut s = String::from("hello");
    let word_index = first_word_slice(&mut s);
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // ^^^^^^^^^ mutable borrow occurs here
    // s.clear();
    println!("slice: {}", word_index);


    // slice as parameter
    let s1 = String::from("hello");
    println!("slice as parameter: {}", first_word_slice_parameter(&s1[..]));
    
    let s2 = "hello";
    println!("slice as parameter: {}", first_word_slice_parameter(s2));

    // slice of array
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    println!("slice of array: {}", slice[0]);

}
// s不可以用, drop()

fn take_ownership(some_str: String){
    println!("take_ownership: {}", some_str);
}


fn makes_copy(some_num: i32){
    println!("makes_copy: {}", some_num);
}

fn gives_ownership() -> String{
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String{
    s
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}


fn calculate_length_ref(s: &String) -> usize {
    // error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
    // `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // s.push_str(", world");
    s.len()
}

fn calculate_length_ref_mut(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

// dangling pointer
// error[E0106]: missing lifetime specifier, ^ expected named lifetime parameter
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// slice
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i
        }
    }
    s.len()
}


// slice
fn first_word_slice(s: &mut String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


// slice
fn first_word_slice_parameter(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}