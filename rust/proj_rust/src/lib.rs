mod front_of_house {
    // error[E0603]: module `hosting` is private
    // mod hosting {
    pub mod hosting {
        // error[E0603]: function `add_to_waitlist` is private
        // fn add_to_waitlist() {
        pub fn add_to_waitlist() {


        }
        fn seat_at_table() {

        }
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}

        // super
        pub mod back_of_house{
            // pub 的 enum 内部也都是pub
            pub enum Appetizer {
                Soup,
                Salad,
            }
            fn fix_incorrect_order(){
                cook_order();
                super::serve_order();
                crate::front_of_house::serving::serve_order();
            }
            fn cook_order(){

            }

            // pub struct 的内部不是pub
            pub struct Breakfast{
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast{
                    Breakfast { 
                        toast: String::from(toast), 
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

        }
    }
}

pub fn eat_at_restaurant(){
    // 使用绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径调用
    front_of_house::hosting::add_to_waitlist();


    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");
}

// use
// use absolute path
use crate::front_of_house::hosting;
// use relative path
// use front_of_house::hosting;
// pub use 重新导出
// pub use crate::front_of_house::hosting;


pub fn eat_at_restaurant_ex(){
    hosting::add_to_waitlist();
}
