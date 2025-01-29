#[cfg(test)]
mod test {

    #[derive(Debug)]
    struct Node {
        id: u8,
        next: Option<Box<Node>>
    }

    #[test]
    fn tests_box_smart_pointer(){
        let chain = Box::new(Node {
            id: 1,
            next: Some(Box::new(Node {
                id: 2,
                next: Some(Box::new(Node {
                    id: 3,
                    next: None,
                }))
            }))
        });
        dbg!(chain);
    }

    #[test]
    fn test_reference_counter(){
        use std::rc::Rc;
        use std::cell::RefCell;

        let a = Rc::new(RefCell::new(Some(20)));
        let b = Rc::clone(&a);
        println!("Var a before: {:?}", a);
        println!("B ref : {:?}", b);
        *a.borrow_mut() = None;
        println!("Var a now: {:?}", a);
        println!("B ref now: {:?}", b);
    }
}