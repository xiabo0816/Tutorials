
// 函数和闭包的定义语法
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|      ->     { x + 1 };
// let add one v4 = |x|               x + 1  ;

use std::{thread, vec};
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout_ex(simulated_user_specified_value, simulated_random_number);

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("Got: {}", val);
    }


}


fn simulated_expensive_calculation(inensity: u32) -> u32 {
    println!("calculating slowly ... ");
    thread::sleep(Duration::from_secs(2));
    inensity
}

fn generate_workout(inensity:u32,random_number:u32){
    if inensity < 25 {
        println!(
            "do {} pushups", 
            simulated_expensive_calculation(inensity)
        );
        println!(
            "do {} situps",
            simulated_expensive_calculation(inensity)
        );
    } else {
        if random_number == 3 {
            println!("take a break");
        } else {
            println!(
                "run for {} minutes",
                simulated_expensive_calculation(inensity)
            );
        }
    }
}

// Fn Trait
// ·Fn traits 由标准库提供
// ·所有的闭包都至少实现了以下 trait 之一:
// - Fn
// - FnMut
// - FnOnce

use std::{collections::HashMap, cmp::Eq, hash::Hash};
// let mut scores: HashMap<String, i32> = HashMap::new();
// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("yellow"), 50);
// println!("{:?}",scores);

struct Cacher<T, U, V> 
    where T: Fn(U) -> V
{
    calculation: T,
    value: HashMap<U, V>,
}

// 通过U作为Hash的key需要实现Eq和Hash两个trait
// 取值返回时，U，V也强制其要求实现Copy trait，因为value函数里面会move U，V类型的值
impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy
{
    fn new(calculation:T)->Cacher<T, U, V>{
        Cacher {
            calculation: calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let value = self.value.get(&arg);
        match value {
            Some(s) => s.clone(),
            None    => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}

// 使用缓存器 (Cacher) 实现的限制
// 1. Cacher 实例假定针对不同的参数 arg，value 方法总会得到同样的值
// -可以使用 HashMap 代替单个值:
// · key: arg 参数
// · value:执行闭包的结果
// 2. 只能接收一个 u32 类型的参数和 u32 类型的返回值

fn generate_workout_ex(inensity:u32,random_number:u32){
    let mut expensive_closure = Cacher::new(|num: u32|{
        println!("calculating slowly ... ");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if inensity < 25 {
        println!("do {} pushups", expensive_closure.value(inensity));
        println!("do {} situps", expensive_closure.value(inensity+1));
    } else {
        if random_number == 3 {
            println!("take a break");
        } else {
            println!("run for {} minutes",expensive_closure.value(inensity));
        }
    }
}




#[cfg(test)]
mod tests{
    use std::vec;

    #[test]
    fn call_with_different_values() {
        let mut c = super::Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_values2() {
        let mut c = super::Cacher::new(|a| a);
        let v1 = c.value("12");
        let v2 = c.value("123");
        assert_eq!(v2, "123");
    }

    #[test]
    fn call_with_different_values3() {
        let mut c = super::Cacher::new(|a| a);
        let mut d = super::Cacher::new(|a: &str|{a.len()});

        c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);

        let v1 = d.value("str123");
        let v2 = d.value("str123456");
        assert_ne!(v1, v2);
        assert_eq!(v2, 9);
    }


    #[test]
    fn use_closure_get_context_variables(){
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));
    }

    #[test]
    fn use_closure_move_context_variables(){
        let x = vec![1,2,3];
        let equal_to_x = move |z| z == x;
        // error[E0382]: borrow of moved value: `x`
        // ^ value borrowed here after move
        // println!("can't use x here: {:?}",x);
        let y = vec![1,2,3];
        assert!(equal_to_x(y));
    }

    #[test]
    fn iterator_demonstration(){
        let v1 = vec![1,2,3];
        // 调用next时，修改了iter的记录位置的变量
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }
    // 几个迭代方法
    // · iter 方法: 在不可变引用上创建迭代器
    // · into_iter 方法: 创建的迭代器会获得所有权
    // · iter_mut 方法: 迭代可变的引用

    #[test]
    fn iterator_sum(){
        let v1 = vec![1,2,3];
        let v1_iter = v1.iter();
        let total:i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map(){
        // 产生其它迭代器的方法
        // 定义在 Iterator trait 上的另外一些方法叫做“迭代器适配器” - 把迭代器转换为不同种类的迭代器
        // 如`map`
        // - 接收一个闭包，闭包作用于每个元素
        // - 产生一个新的选代器
        let v1 = vec![1,2,3];
        // collect 方法:消耗型适配器，把结果收集到一个集合类型中
        let v1:Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v1, vec![2,3,4]);
    }


    // filter 方法:
    #[derive(PartialEq, Debug)]
    struct Shoe{
        size:u32,
        style:String
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|x| x.size == shoe_size).collect()
    }

    #[test]
    fn iterator_filter(){
        // filter 方法:
        // - 接收一个闭包
        // - 这个闭包在遍历迭代器的每个元素时，返回 bool 类型
        // - 如果闭包返回 true:当前元素将会包含在 filter 产生的选代器中如果闭包返回 false: 当前元素将不会包含在 filter 产生的迭代器中
        
        let shoes = vec![
            Shoe{
                size:10,
                style:String::from("sneaker"),
            },
            Shoe{
                size:13,
                style:String::from("sandal"),
            },
            Shoe{
                size:10,
                style:String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe{
                    size:10,
                    style:String::from("sneaker"),
                },
                Shoe{
                    size:10,
                    style:String::from("boot"),
                },
            ]
        )
    }

    
    
    struct Counter{
        count:u32
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            }else {
                None
            }
        }
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods(){
        let sum: u32 = Counter::new()
            // 1,2,3,4,5
            .zip(Counter::new().skip(1))
            // 2,3,4,5,None
            .map(|(a,b)| a*b)
            // 2,6,12,20
            .filter(|x| x%3 == 0)
            // 6,12
            .sum();
            // 18
        assert_eq!(18, sum);
    }
}

