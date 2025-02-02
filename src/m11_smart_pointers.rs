#[cfg(test)]
mod test {
    use std::rc::Weak;


    #[test]
    fn tests_box_smart_pointer(){
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Node {
            id: u8,
            next: Option<Box<Node>>
        }

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

    fn tests_reference_counter(){
        // Reference Counter digunakan agar dapat membuat co-owner
        // Sehingga 1 data bisa dimiliki oleh lebih dari 1 owner (co-owner)
        use std::rc::Rc;
        let a: Rc<i32> = Rc::new(32);
        let b: Rc<i32> = Rc::clone(&a); // Menambahkan Reference Counter (Reference yang mengarah ke data yang sama dengan a)

        println!("Nilai Reference A: {}", a);
        println!("Nilai Reference B: {}", b);

        println!("Alamat Reference A: {:p}", a);
        println!("Alamat Reference B: {:p}", b);

    }

    fn tests_reference_counter_refcell(){
        use std::cell::RefCell;
        use std::rc::Rc;

        // Disini Rc bertindak sebagai counter untuk menyimpan list owner
        // Rc berperan agar 1 data dapat memiliki beberapa owner (co-owner) sekaligus

        // RfCell disini berperan untuk mengawasi borrower.
        // Ada kalanya salah satu owner ingin mengganti nilai dari data bersama
        // disini RfCell berfungsi menjaga agar proses borrower memungkinkan lebih dari 1 borrower 
        // tapi tidak melanggar aturan borrower

        let a:Rc<RefCell<i32>> = Rc::new(RefCell::new(33));
        let b:Rc<RefCell<i32>> = Rc::clone(&a);

        println!("Nilai a dan b before: {:?}, {:?}", a, b);

        *b.borrow_mut() = 11;

        println!("Nilai a dan b after: {:?}, {:?}", a, b);
    }

    fn tests_ref_counter_with_weak(){

        use std::rc::Rc;
        use std::cell::RefCell;

        #[derive(Debug)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Furniture>>,
        }

        #[derive(Debug)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House> 
            // menggunaka Rc<House> dapat mengakibatkan memory leak
            // karena House dan Furniture saling mengikat (strong_count)
            // yang mengakibatkan cycle reference terjadi 
            // dimana house tidak bisa di hapus karena mengikat furniture dan furniture mengikat house
            // sehingga reference count tidak pernah bernilai 0.
            // maka itu kita menggunakan Weak sehingga disini furniture tidak akan menambah reference count
            // dan ketika objek aslinya dihapus maka Weak otomatis melepaskan
        }

        let house1: Rc<House> = Rc::new(House {
            address_number: 20,
            street: "Jl. SM Raja".to_string(),
            furniture: RefCell::new(vec![]),
        });

        let table: Furniture  = Furniture {
            id: "table1".to_string(),
            description: "Table".to_string(),
            house: Rc::downgrade(&house1)
        };

        let desk: Furniture  = Furniture {
            id: "desk1".to_string(),
            description: "Desk".to_string(),
            house: Rc::downgrade(&house1)
        };

        house1.furniture.borrow_mut().push(table);
        house1.furniture.borrow_mut().push(desk);

        println!("{:?}", house1);



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

        tests_reference_counter();
        tests_reference_counter_refcell();
        tests_ref_counter_with_weak();
    }
}