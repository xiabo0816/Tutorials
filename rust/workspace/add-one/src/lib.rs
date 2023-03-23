pub fn addone(x: i32) -> i32 {
    x+1
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_works(){
        assert_eq!(3, addone(2));
    }
}