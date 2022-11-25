// cargo new autotest --lib
// cargo test

// 测试函数体(通常) 执行的 3a操作:
// - 准备数据/状态，arrange
// - 运行被测试的代码，action
// - 断言 (Asser)结果

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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