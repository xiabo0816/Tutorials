pub trait Summary{
    fn summarize_author(&self) -> String;
    // trait 的默认实现
    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify1(item1: impl Summary, item2: impl Summary){
    println!("Breaking news! {}",item1.summarize());
}

pub fn notify_traitbound<T: Summary>(item1: T, item2: T){
    println!("Breaking news! {}",item1.summarize());
}

// 多个trait
use std::fmt::Display;
pub fn notify2(item1: impl Summary + Display, item2: impl Summary){
    println!("Breaking news! {}",item1.summarize());
}

pub fn notify3<T: Summary + Display>(item1: T){
    println!("Breaking news! {}",item1.summarize());
}

// where
use std::fmt::Debug;
pub fn notify4<T:Summary + Display, U: Clone + Debug>(a:T, b:U) -> String{
    format!("Breaking news! {}", a.summarize())
}

pub fn notify5<T, U>(a:T, b:U) -> String
    where 
        T:Summary + Display, 
        U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}

pub fn notify6(s: &str) -> impl Summary{
    Tweet{
        username: String::from(s),
        content: String::from(";klj"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x:T, y:T) ->Self {
        Self {x,y}
    }
}

// 有Display + PartialOrd时，才有cmp_display
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y{
            println!("x = {}", self.x);
        }else{
            println!("y = {}", self.y);
        }
    }
}

// blanket implementations
// 3.to_string()

// struct lifetime
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a>{
    fn level(&self) ->i32{
        3
    }

    fn announce_and_return_part(&self, announcement:&str)->&str{
        println!("impl fn lifetime, {}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T>
    (x:&'a str, y:&'a str, ann:T) -> &'a str
where
    T: Display,
{
    println!("generics, trait bound, lifetime, {}", ann);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
