// cargo new autotest --lib
// cargo test
// cargo test <test fn/module name prefix>

// 其他命令
// cargo test -- help
// cargo test -- --help
// cargo test -- --test-threads=1
// cargo test -- --show-output
// cargo test -- --ignored

// 测试函数体(通常) 执行的 3a操作:
// - 准备数据/状态，arrange
// - 运行被测试的代码，action
// - 断言 (Asser)结果

// Rust 对测试的分类:
// - 单元测试
// - 集成测试
// 单元测试:
// - 小、专注
// - 一次对一个模块进行隔离的测试
// - 可测试 private 接口
// 集成测试:
// - 在库外部。和其它外部代码一样使用你的代码
// - 只能使用 public 接口
// - 可能在每个测试中使用到多个模块

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// cfg 告诉 Rust 下面的条目只有在指定的配置选项下才被包含
// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("asdf");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{
            height:8,
            width:7,
        };
        let smaller = Rectangle{
            height:2,
            width:3,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{
            height:8,
            width:7,
        };
        let smaller = Rectangle{
            height:2,
            width:3,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_not_adds_two(){
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contain_name(){
        // test tests::greeting_contain_name ... FAILED
        let result = greeting("carol");
        assert!(
            result.contains("caro1l"),
            "Didn't contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "should panic, value > 100, got")]
    fn greater_than_100() {
        // test tests::greater_than_100 - should panic ... ok
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "should panic, value < 1, got")]
    fn less_than_1() {
        // test tests::less_than_1 - should panic ... ok
        Guess::new(-1);
    }

    // use Result<T,E> test
    #[test]
    fn it_works_22() -> Result<(), String> {
        if 2+2==4{
            Ok(())
        }else{
            Err(String::from("it works 22"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test(){
        // test tests::expensive_test ... ignored
        assert_eq!(5, 1+1+1+1+1);
    }

    #[test]
    fn internal_adder_test(){
        assert_eq!(4, internal_adder(2,2));
    }
}


// hello_rust/struct.rs
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


pub fn add_two(a:i32)->i32{
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}


pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess{
        if value < 1 {
            panic!("should panic, value < 1, got: '{}'", value);
        } else if value > 100 {
            panic!("should panic, value > 100, got: '{}'", value);
        }
        Guess {
            value
        }
    }
}

// rust test支持测试私有函数的测试
fn internal_adder(a:i32,b:i32)->i32{
    a+b
}