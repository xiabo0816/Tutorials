//! # 生成 HTML 文档的命令
//! `cargo doc --open`
//! - 它会运行 rustdoc 工具 (Rust 安装包自带)
//! - 把生成的 HTML 文档放在 target/doc目录下

/// Adds on to the number given
/// 
/// # Examples
/// ```rust
/// let arg = 5;
/// let answer = mycrate::add_one(arg);
/// assert_eq!(6, answer);
/// ```
/// 
/// # Panics
/// 函数可能发生 panic 的场景
/// 
/// # Errors
/// 如果函数返回 Result，描述可能的错误种类，以及可导致错误的条件
/// 
/// # Safety
/// 如果函数处于 unsafe 调用，就应该解释函数 unsafe 的原因，以及调用者确保的使用前提。
/// 
/// 
pub fn add_one(x:i32) ->i32{
    x+1
}



// - 使用 pub use: 
// 可以重新导出，
// 也就是创建一个与内部私有结构不同的对外公共结构
// 不需要重新组织内部代码结构
// `cargo doc`会添加`Re-exports`
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
pub mod kinds{
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils{
    use crate::kinds::*;

    pub fn mix(c1:PrimaryColor, c2:PrimaryColor) -> SecondaryColor{
        SecondaryColor::Green
    }
}