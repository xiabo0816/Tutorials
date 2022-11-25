fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 49, 23, 199, 12];
    let result = largest(&number_list);
    println!("largest {}", result);

    let number_list = vec![34, 49, 23, 6000, 12];
    let result = largest(&number_list);
    println!("largest {}", result);


    // generics
    let integer = Point{x:1,   y:2};
    let float   = Point{x:1.2, y:2.2};
    // error[E0308]: mismatched types
    // ^ expected floating-point number, found integer
    // let integer_float  = Point{x:1.2, y:2};
    let integer_float2 = Point2{x:1.2, y:2};

    // 
    let integer = Point{x:1,   y:2};
    println!("integer.x = {}", integer.x());
    println!("integer.x1 = {}", integer.x1());

    // mix up
    let p1 = Point2{x:5,y:4};
    let p2 = Point2{x:"a",y:"b"};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // trait
    // error[E0599]: no method named `summarize` found for struct `Tweet` in the current scope
    use generics_rust::Summary;
    use generics_rust::Tweet;
    let tweet = Tweet{
        username: String::from("asdf"),
        content: String::from(";klj"),
        reply: false,
        retweet: false,
    };
    println!("tweet.summarize: {}", tweet.summarize());

    // trait where
    let a = generics_rust::notify6("ffff");
    println!("trait where: {}", a.summarize());

    // largest type
    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest_type(&str_list);
    println!("The largest word is {}", result);

    // life cycle
    {
        let r = 1;
        {
            let x = 5;
            // error[E0597]: `x` does not live long enough
            // ^^ borrowed value does not live long enough
            // r = &x;
        }
        println!("r: {}",r);
    }

    // life cycle in fn
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("life cycle in fn, {}", result);

    // lifetime in fn 
    let string1 = String::from("abcd");
    let result  = string1;
    {
        let string2 = String::from("yz");
        // error[E0597]: `string2` does not live long enough
        // ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
        // result = longest(string1.as_str(), string2.as_str());
    }
    println!("lifetime in fn, {}", result);

    // struct lifetime
    let novel = String::from("struct. lifetime");
    let first_sentence = novel.split('.').next().expect("no '.'");
    let i = generics_rust::ImportantExcerpt{
        part: first_sentence
    };
}

fn largest(list: &[i32]) -> i32{
    let mut largest = list[0];
    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    // for item in list{
    //     if *item > largest{
    //         largest = *item;
    //     }
    // }
    largest
}

fn largest_type<T>(list: &[T]) -> &T 
    where T: PartialOrd + Clone
{
    let mut largest = &list[0];
    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x:T,
    y:T
}

// 把T放在 impl 关键字后，表示在类型T上实现方法
impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

// 只针对具体类型实现方法(其余类型没实现方法) 
impl Point<i32> {
    fn x1(&self) -> &i32{
        &self.x
    }
}


struct Point2<T, U> {
    x:T,
    y:U
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W>{
        Point2{
            x: self.x,
            y: other.y,
        }
    }
}

// error[E0106]: missing lifetime specifier
// ^ expected named lifetime parameter
// fn longest(x:&str, y:&str) ->&str {
fn longest<'a>(x:&'a str, y:&'a str) ->&'a str {
    // 生命周期'a的实际生命周期是:x 和y 两个生命周期中较小的那个
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

// error[E0515]: cannot return reference to local variable `result`
// ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
// fn longest_hanging<'a>(x:&'a str, y:&'a str) ->&'a str {
//     let result = String::from("abc");
//     result.as_str()
// }

// 生命周期省略规则
// 假设我们是编译器:
// fn first_word(s: &str) -> &str f
// fn first_word<a>(s: &a str) -> &str 
// fn first_word<'a>s: &'a str) -> &a str {
// fn longest(x: &str, y: &str) -> &str {
// fn longest<'a, 'b>(x: &'a str, y: &b str) -> &str [

