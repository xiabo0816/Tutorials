// RefCell<T>
// 内部可变性 (interior mutability)
// 内部可变性是 Rust 的设计模式之一
// - 它允许你在只持有不可变引用的前提下对数据进行修改
//  -数据结构中使用了 unsafe 代码来绕过 Rust 正常的可变性和借用规则
pub trait Messenger {
    // RefCell<T> &self
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a+Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl <'a, T> LimitTracker<'a, T> 
where
    T: Messenger,
{
    pub fn new(messenger: &T, max:usize) -> LimitTracker<T> {
        LimitTracker { 
            messenger, 
            value: 0, 
            max
        }
    }

    pub fn set_value(&mut self, value:usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0{
            self.messenger.send(">=1");
        }else if percentage_of_max >= 0.9 {
            self.messenger.send("90%");
        }else if percentage_of_max >= 0.75 {
            self.messenger.send("75%");
        }
        
    }
    
}

#[cfg(test)]
mod tests{
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger{
        // RefCell<T>
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger { 
                // RefCell<T>
                sent_messages: RefCell::new(vec![]) 
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, msg: &str) {
            // RefCell<T>
            self.sent_messages.borrow_mut().push(String::from(msg));
            // println!("{}", self.sent_messages.borrow().len());
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_msg(){
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // RefCell<T>
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}