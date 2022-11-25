
struct User{
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, name: String) -> User{
    User{
        name,
        email,
        sign_in_count: 0,
        active: true,
    }
}

// tuple struct
struct Color(i32, i32, i32);

// unit like struct


// demo
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}


fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main(){
    let user1 = build_user(String::from("asdf"), String::from("asdf"));
    let mut user2 = User{
        name: String::from("asdf"),
        email: String::from("asdf"),
        ..user1
    };
    user2.name = String::from("asdf");
    println!("{},{},{},{}", user2.name, user2.email, user2.sign_in_count, user2.active);

    let black = Color(0,0,0);
    println!("{}", black.0);

    // demo
    let rect = Rectangle{
        width: 20,
        height: 30,
    };
    println!("area(&rect): {}", area(&rect));
    println!("{:#?}", rect);
    
    println!("rect.area(): {}", rect.area());


    let rect2 = Rectangle{
        width: 30,
        height: 40,
    };
    println!("{:#?}", rect2);
    println!("rect.can_hold(&rect2): {}", rect.can_hold(&rect2));
    println!("rect2.can_hold(&rect): {}", rect2.can_hold(&rect));

    let s = Rectangle::square(20);
    println!("{:#?}", s);

    // enum
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;
    route(four);
    route(six);
    route(IpAddrKind::V4);

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_data = IpAddrWithData::V4(127,0,0,1);
    let loopback_data = IpAddrWithData::V6(String::from("::1"));

    // enum with 
    let q = Message::Quit;
    let m = Message::Move{x:12, y:23};
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0,244,244);

    // enum with function
    m.call();

    // Option
    let some_number = Some(5);
    let some_string = Some("asdf");
    let absent_number: Option<i32> = None;

    // Option<T> + T
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // error[E0277]: cannot add `Option<i8>` to `i8`
    // ^ no implementation for `i8 + Option<i8>`
    // let sum = x + y;


    // match
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));

    // 匹配Option<T>
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);

    // _ for match
    let v = 0u8;
    match v {
        1 => println!("one"),
        3 => println!("one"),
        5 => println!("one"),
        7 => println!("one"),
        _ => println!("_ for match"),
    }

    // if let
    let v = Some(3u8);
    match v {
        Some(3) => println!("match three"),
        _ => println!("match others"),
    }

    if let Some(3) = v {
        println!("if let three");
    } else {
        println!("if let others");
    }

}


// enum
enum IpAddrKind{
    V4,
    V6
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum IpAddrWithData{
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){

    }
}

fn route(ip_kind: IpAddrKind){

}



// match
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny, Nickel, Dime, Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny   => {
            println!("Penny");
            1
        },
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("Quarter {:?}", state);
            25
        },
    }
}

// 匹配Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // error[E0004]: non-exhaustive patterns: `None` not covered
        // ^ pattern `None` not covered
        None    => None,
        Some(i) => Some(i+1),
    }
}


