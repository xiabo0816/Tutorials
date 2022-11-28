// tests/ 目录很特殊，cargo test才会处理 ?????

// cargo test itgt_it_adds_two
// cargo test --test intergration_test

use autotest;
mod common;

#[test]
fn itgt_it_adds_two(){
    let a = common::setup();
    assert_eq!(a+2, autotest::add_two(a));
}


// 如果项目是 binary crate，只含有 src/main.rs 没有 src/lib.rs:
// - 不能在 tests目录下创建集成测试
// -无法把 main.rs 的函数导入作用域
// 只有 library crate 才能暴露函数给其它 crate 用
// binary crate 意味着独立运行