fn main() {
    println!("Hello, world!");

    // Box<T>
    let b = Box::new(5);
    println!("box, b={}", b);

    // Cons List
    // error[E0072]: recursive type `List` has infinite size
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    use crate::List::{Cons, Nil};
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, Box::new(Nil)))
            ))
        );


    // Deref Trait
    // 实现 Deref Trait 使我们可以自定义解引用运算符*的行为。
    // 通过实现 Deref，智能指针可像常规引用一样来处理
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    // *(y.deref())
    assert_eq!(5, *y);


    // deref coercion
    // 解引用与可变性
    // 可使用 DerefMut trait 重载可变引用的 * 运算符
    // 在类型和 trait 在下列三种情况发生时，Rust 会执行 deref coercion:
    // - 当T: Deref<Target=U>，允许 &T 转换为 &U
    // - 当T: DerefMut<Target=U>，允 &mut T 转换为 &mut U
    // - 当T: Deref<Target=U>，允许 &mut T 转换为 &U
    let m = MyBox::new(String::from("rust"));
    // &m: &MyBox<String>
    // deref &String
    // deref &str
    hello(&m);
    hello(&(*m)[..]);
    hello("str");


    // Drop Trait
    {
        let c = CustomSmartPoiner{data:String::from("111")};
        let d = CustomSmartPoiner{data:String::from("222")};
        println!("CustomSmartPoiner created")
        // error[E0040]: explicit use of destructor method
        // explicit destructor calls not allowed
        // c.drop()

    }
    {
        let e = CustomSmartPoiner{data:String::from("333")};
        let f = CustomSmartPoiner{data:String::from("444")};
        println!("CustomSmartPoiner created");
        drop(e);
        // error[E0040]: explicit use of destructor method
        // explicit destructor calls not allowed
        // c.drop()

    }

    // Rc<T>引用计数智能指针, ref count
    // 只适用于单线程
    let a = Cons(5, 
        Box::new(Cons(10, 
            Box::new(Cons(11, Box::new(Nil)))
            ))
        );

    let b = Cons(3, Box::new(a));
    // value used here after move
    // let c = Cons(4, Box::new(a));
    
    use crate::ListRc::{ConsRc, NilRc};
    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("rc a {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("rc b {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("rc c {}", Rc::strong_count(&a));
    }
    println!("rc c {}", Rc::strong_count(&a));
    

    // 将 Rc<T> 和 RefCell<T> 结合使用
    use crate::List3::{Cons3, Nil3};
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));
    let b = Cons3(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(9)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    // 循环引用可导致内存泄漏
    use crate::List4::{Cons4, Nil4};
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));
    println!("a rc count {}", Rc::strong_count(&a));
    println!("a.next {:?}", a.tail());

    let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count {}", Rc::strong_count(&a));
    println!("b rc count {}", Rc::strong_count(&b));
    println!("b.next {:?}", b.tail());

    // 取出a的第二个元素
    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count {}", Rc::strong_count(&a));
    println!("b rc count {}", Rc::strong_count(&b));
    
    // cycle ref make mem leak
    // println!("a.next {:?}", a.tail());

    // Weak<T>
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node{
            value:5, 
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
        // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("branch strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    // leaf.parent虽然引用了branch, 但是这种weak<T>弱引用不影响释放
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

}

// error[E0072]: recursive type `List` has infinite size
// enum List{
//     Cons(i32, List),
//     Nil,
// }

enum List{
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T> (T);
impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
    
}

use std::borrow::Borrow;
use std::cell::{RefCell, Ref};
// 标准库中的 Deref trait 要求我们实现一个 deref 方法:
// - 该方法借用 self
// - 返回一个指向内部数据的引用
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    // 定义关联类型
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

// deref coercion
fn hello(name:&str){
    println!("hello, {}", name);
}


// Drop Trait
// 实现 Drop Trait，可以让我们自定义当值将要离开作用域时发生的动作
// - 例如:文件、网络资源释放等
// - 任何类型都可以实现 Drop trait
// Drop trait 只要求你实现 drop 方法
// - 参数: 对self 的可变引用
// Drop trait 在预导入模块里 (prelude)
struct CustomSmartPoiner{
    data:String,
}

impl Drop for CustomSmartPoiner{
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoiner with data:`{}`", self.data);
    }
}

// Rc<T>引用计数智能指针, ref count
use std::rc::{Rc, Weak};
use std::vec;
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc
}


// 将 Rc<T> 和 RefCell<T> 结合使用
// 实现一个拥有多重所有权的可变数据
#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

// 循环引用可导致内存泄漏
#[derive(Debug)]
enum List4 {
    Cons4(i32, RefCell<Rc<List4>>),
    Nil4,
}
use crate::List4::{Cons4, Nil4};

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match self {
            Cons4(_, item) => Some(item),
            Nil4 => None,
        }
    }
    
}

// Weak<T>
#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

