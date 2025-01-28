#[cfg(test)]
mod test {

    fn example_1(){
        let bob: i32 = 23;
        let compare_age : &i32;
        {
            let _john: i32 = 24;
            compare_age = add(&bob, &_john);
        }
    
        println!("{}", compare_age);
    
        fn add<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
            if x < y {
                &13
            } else {
                &10
            }
        }
    }

    #[test]
    fn tests_lifetimes(){
        example_1();
    }
}