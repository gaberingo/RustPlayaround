#[cfg(test)]
mod test_collection {

    use std::collections::{HashMap, HashSet};
    use std::any::Any;


    fn tests_hashmap(){
        let key1 = "Name";
        let key2 = "Age";

        // Hashmap { Key, Value}
        let mut info: HashMap<&str, Box<dyn Any>> = HashMap::new();

        info.insert(key1, Box::new("Gabe Siringoringo"));
        info.insert(key2, Box::new(24));


        let name = info.get(key1).unwrap().downcast_ref::<&str>();
        if let Some(x) = name {
            println!("Name: {}", x);
        }
    }

    fn tests_hashset(){
        let val1 = "Gabe";
        let val2 = "Siringoringo";

        let mut var_hashset= HashSet::new();

        var_hashset.insert(val1);
        var_hashset.insert(val2);

        dbg!(&var_hashset);
        dbg!(var_hashset.contains("Gabe"));
    }

    #[test]
    fn tests(){
        println!("++++++++++++++++++++++++++++++++++++++++++");
        tests_hashmap();
        println!("++++++++++++++++++++++++++++++++++++++++++");
        tests_hashset();
    }
}