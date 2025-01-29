#[cfg(test)]
mod test {
    #[macro_export]
    macro_rules! my_vec {
        () => {
            Vec::new()
        };
        ($el: expr, $n: expr) => {
            std::vec::from_elem($el, $n)
        };
        ($($val: expr), *) => {
            {

                let mut arr = Vec::new();
                $(
                    arr.push($val);
                )*
                arr
            }
        }
    }

    #[test]
    fn test_decl_macros(){
        dbg!("Hellow World");
        let mut from_new_macros: Vec<i32> = my_vec!();
        from_new_macros.push(12);
        from_new_macros.push(15);
        from_new_macros.push(11);
        from_new_macros.push(45);
        from_new_macros.push(22);

        let slice1: &[i32] = &mut from_new_macros[..2];
        dbg!(slice1);
        dbg!(from_new_macros);
    }
}