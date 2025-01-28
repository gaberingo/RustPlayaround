#[cfg(test)]
mod test {

    // Pattern match with Option enum
    fn example_1() {
        let a = Some(12);
        let b: Option<&str> = None;

        // Contoh 1
        match a {
            Some(x) => println!("match var -> {}", x),
            None => panic!("There is something wrong")
        };

        // Contoh 2
        let res = match b {
            None => "There is something wrong",
            Some(_) => "Some Value",
        };

        println!("let var = match -> {}", res);

        if let Some(x) = a {
            println!("if let some -> {}", x);
        }

        if let None = b {
            println!("let None -> None");
        }

    }

    // Pattern match with Result enum
    fn example_2(){
        let res1: Result<i32, &str> = Ok(12);
        let res2: Result<i32, &str> = Err("Cause this error");

        println!("Using match");
        match res1 {
            Ok(x) => println!("{:?}", x),
            Err(_) => println!("Something Wrong")
        };

        match res2 {
            Ok(_) => println!("This will not executed"),
            Err(err) => println!("some thing wrong. msg: {}", err)
        };

        let res = match res1 {
            Ok(val) => val,
            Err(_) => panic!("Something wrong")
        };
        println!("From let var -> {}", res);

        if let Ok(x) = res1 {
            println!("Ok from if let -> {}", x)
        }

        if let Err(e) = res2 {
            println!("Err from if let -> {}", e)
        }
    }

    // Mixed data enum
    fn example_3(){
        enum Action<'a> {
            Attack{dmg: i32, resis: i32},
            Speak(&'a str),
            Move{x: i32, y: i32, z: i32},
            Died
        }

        let action1: Action = Action::Move { x: 23, y: 11, z: 67 };
        let action2: Action = Action::Died;
        let action3: Action = Action::Speak("Haloo semua");
        let action4: Action = Action::Attack { dmg: 100, resis: 10 };

        fn action_info(act: Action){
            match act {
                Action::Attack { dmg, resis } => println!("Hero menyerang dengan damage {}", dmg-resis),
                Action::Speak(word) => println!("Hero mengatakan {}", word),
                Action::Move { x, y, z } => println!("Hero bergerak ke koor x: {}, y: {}, x: {}", x, y, z),
                Action::Died => println!("Hero mati")
            }
        }

        action_info(action1);
        action_info(action2);
        action_info(action3);
        action_info(action4);
    }

    // Pattern With Guard
    fn example_4(){
        let xy = (12, -12);
        let res: Result<i32, &str> = Ok(8);

        match xy {
            (x, y) if x + y == 0 => println!("Menghasilkan 0"),
            (x,y) if x > y => println!("{} lebih besar dari {}", x, y),
            (x,y) if x + y % 2 == 0 => println!("{} ditambah {} menghasilkan bilangan genap", x, y),
            _ => println!("_")
            
        };

        match res {
            Ok(x) if x > 10 => println!("{} lebih besar dari 10", x),
            Ok(x) if x % 2 == 0 => println!("{} adalah bilangan genap", x),
            Ok(x) => println!("Nilainya {}", x),
            Err(_) => panic!("Ada Error")
        }
    }

    // Pattern with struct
    fn example_5(){
        #[derive(Debug)]
        struct Employee<'a> {
            name: &'a str,
            role: Role
        }

        #[derive(Debug, PartialEq)]
        #[allow(dead_code)]
        enum Role {
            Developer,
            PM,
            CEO
        }

        let human1 = Employee {
            name: "Gabe Ringo",
            role: Role::Developer
        };

        match human1 {
            Employee{name, role} if name.contains("Gabe") && role == Role::Developer => println!("This is me!!, im a {:?}", role),
            Employee{name, role: Role::CEO} => println!("I am a CEO. My name is {}", name),
            Employee{name, role} => println!("Nama: {}, Role: {:?}", name, role)
        }

        let human2 = Employee {
            name: "ricky halim",
            role: Role::Developer
        };

        if let Employee{name, role: Role::Developer} = human2 {
            println!("nama saya {}. Sebagai Developer", name)
        }

        let human3: Employee = if let Employee {name, role: Role::Developer} = human2 {
            Employee {name: name, role: Role::CEO}
        } else {
            Employee {name: "Gabe", role: Role::CEO}
        };

        println!("{:?}", human3)
    }

    #[test]
    fn tests_patterns(){
        println!("################################################");
        example_1();
        println!("################################################");
        example_2();
        println!("################################################");
        example_3();
        println!("################################################");
        example_4();
        println!("################################################");
        example_5();
    }
}