fn main() {
    // vector
    let v: Vec<i32> = Vec::new();
    println!("{:?}",v);
    
    let v = vec![1,2,3];
    println!("{:?}",v);

    // vector update 
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // vector delete
    let v = vec![1,2,3,4];
    let third: &i32 = &v[2];
    println!("third {}", third);

    match v.get(2){
        Some(third) => println!("third {}", third),
        None        => println!("no third"),
    }

    // thread 'main' panicked at 'index out of bounds:
    // let third: &i32 = &v[100];
    match v.get(100){
        Some(hundred) => println!("hundred {}", hundred),
        None        => println!("no hundred"),
    }

    println!("Hello, world!");

    // 所有权和借用规则
    let mut v = vec![1,2,3,4];
    let first = &v[0];
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // ^^^^^^^^^ mutable borrow occurs here
    // v.push(6);
    println!("first {}",first);

    // 遍历vector的值
    let v = vec![1,2,3];
    for i in &v {
        println!("{}", i);
    }

    // 遍历vector的值
    let mut v = vec![1,2,3];
    for i in &mut v {
        *i += 30;
    }
    for i in &v {
        println!("{}", i);
    }

    // Vector + enum
    // 实现同一Vector内，不同的数据类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("asdf")),
        SpreadsheetCell::Float(3.14),
    ];
    println!("{:?}",row);


    // String 和 &str
    // OsString, OsStr, CString, CStr
    let mut s = String::new();

    let data = "init ocntents";
    let s = data.to_string();
    let ss = "init ocntents".to_string(); 

    let mut s = String::from("init contentsa阿斯蒂芬");
    s.push_str("bar");
    println!("{}",s);

    s.push_str(".");
    println!("{}",s);
    
    let s2 = String::from("asdf");
    let s3 = s + &s2;
    println!("{}",s3);
    println!("{}",s2);
    // error[E0382]: borrow of moved value: `s
    // ^ value borrowed here after move
    // println!("{}",s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);
    let s3 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s3);

    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // ^^^^^ `String` cannot be indexed by `{integer}`
    // let h = s3[0];

    let len = String::from("Hola").len();
    println!("Hola: {}", len);
    // unicode 标量值
    let len = String::from("阿斯蒂芬").len();
    println!("阿斯蒂芬: {}", len);
    // 字节(Bytes)、标量值(Scalar Values)、字形簇(Grapheme Clusters)

    let w = "阿斯蒂芬";
    println!("阿斯蒂芬: {}", w.len());
    for b in w.bytes() {
        println!("{}", b);
    }
    for b in w.chars() {
        println!("{}", b);
    }

    let s = &w[0..3];
    println!("{}", s);

    // thread 'main' panicked at 'byte index 4 is not a char boundary;
    // let s = &w[0..4];
    // println!("{}", s);

    // HashMap
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("{:?}",scores);

    // vector, collect, hashmap
    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10,39];
    // error[E0282]: type annotations needed
    // let scores = teams.iter().zip(initial_scores.iter()).collect();
    let scores:HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}",scores);

    // hashmap ownership
    let field_name = String::from("aaa");
    let field_value = String::from("bbb");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}",map);
    // error[E0382]: borrow of moved value: `field_value`
    // ^^^^^^^^^^^ value borrowed here after move
    // println!("{},{}", field_name, field_value);

    let field_name1 = String::from("aaaa");
    let field_value1 = String::from("bbbb");
    let mut map1 = HashMap::new();
    map1.insert(&field_name1, &field_value1);
    println!("{:?}",map1);
    println!("{},{}", field_name1, field_value1);


    // hashmap get
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 33);
    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    match score{
        Some(s) => println!("{}", s),
        None    => println!("none"),
    };

    for (k, v) in &scores {
        println!("{}:{}",k,v);
    }

    // update hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    println!("{:?}",scores);
    scores.insert(String::from("blue"), 100);
    println!("{:?}",scores);


    // hashmap entry
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.entry(String::from("blue")).or_insert(30);
    scores.entry(String::from("yellow")).or_insert(310);
    println!("{:?}",scores);

    let e = scores.entry(String::from("yellow"));
    println!("{:?}",e);
    let e = scores.entry(String::from("green"));
    println!("{:?}",e);
    
    let text = "h e l l o w o r l d";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        println!("{:#?}",count);
        *count += 1;
    }
    println!("{:#?}",map);

    // hasher
    

}

#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
