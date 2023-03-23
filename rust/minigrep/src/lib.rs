use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // .expect(format!("Someting went wrong reading the file: {}", filename).as_str());
    let results = if config.case_sensitive {
        search(config.query.as_str(), contents.as_str())
    }else{
        search_case_insensitive(config.query.as_str(), contents.as_str())
    };
    println!("{:?}", results);
    Ok(())
}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3{
    //         return Err("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //     // println!("{:?}", query);
    //     // println!("{:?}", filename);
    //     // $env:CASE_INSENTITIVE = '1' | cargo run
    //     let case_sensitive = env::var("CASE_INSENTITIVE").is_err();
    //     Ok(Config{query, filename, case_sensitive})
    // }

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot get query")
        };
        let filename =  match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot get filename")
        };
        // println!("{:?}", query);
        // println!("{:?}", filename);
        // $env:CASE_INSENTITIVE = '1' | cargo run
        let case_sensitive = env::var("CASE_INSENTITIVE").is_err();
        Ok(Config{query, filename, case_sensitive})
    }
}

// pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
//     let mut results = Vec::new();
//     for line in contents.lines(){
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

// using Iterator
pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

// pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
//     let mut results = Vec::new();
//     let query = query.to_lowercase();
//     for line in contents.lines(){
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }
//     results
// }

// using Iterator
pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// 测试驱动开发TDD (Test-Driven Development)
// 。编写一个会失败的测试，运行该测试，确保它是按照预期的原因失败·编写或修改刚好足够的代码，让新测试通过
// 。重构刚刚添加或修改的代码，确保测试会始终通过
// 。返回步骤1，继续

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "the";
        let contents = "\
sadf
asdf
the asdf
THE
";
        assert_eq!(
            vec!["the asdf"], 
            search(query, contents),
        );
    }

    #[test]
    fn case_sensitive(){
        let query = "the";
        let contents = "\
sadf
asdf
the asdf
THE
";
        assert_eq!(
            vec!["the asdf"], 
            search(query, contents),
        );
    }


    #[test]
    fn case_insensitive(){
        let query = "thE";
        let contents = "\
sadf
asdf
the asdf
THE
";
        assert_eq!(
            vec!["the asdf", "THE"], 
            search_case_insensitive(query, contents),
        );
    }
}