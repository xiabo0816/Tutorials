// cargo run <query> <filename>
// cargo run to poem.txt
// $env:CASE_INSENTITIVE = '1' | cargo run to poem.txt
// cargo run > output.txt
// cargo run to poem.txt > output.txt


// v0.1
// use std::env;
// use std::fs;
// fn main() {
//     println!("Hello, world!");
//     let args: Vec<String> = env::args().collect();
//     // env::args_os()
//     let query = &args[1];
//     let filename = &args[2];
//     println!("{:?}", query);
//     println!("{:?}", filename);
//     let contents = fs::read_to_string(filename)
//     .expect(format!("Someting went wrong reading the file: {}", filename).as_str());
//     println!("{:?}", contents);
// }

// 每个函数只做一个功能
// 分离配置变量和具体业务逻辑变量，配置变量收入结构体
// 打印信息内容单一
// 集中错误处理代码


// 二进制程序关注点分离的指导性原则
// 将程序拆分为 main.rs 和 lib.rs，将业务逻辑放入 lib.rs
// 当命令行解析逻辑较少时，将它放在 main.rs 也行
// 当命令行解析逻辑变复杂时，需要将它从 main.rs 提取到 lib.rs
// v0.2
use minigrep::Config;
use std::env;
use std::process; 
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err|{
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}




