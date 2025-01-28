#[cfg(test)]
mod test {

    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        active: bool,
        count: i32
    }

    impl User {
        fn new_user() -> Self {
            Self {
                username: String::new(),
                email: String::new(),
                active: true,
                count: 1
            }
        }

        fn increment_count(&mut self, number: i32){
            self.count += number;
        }

        fn show_info(&self){
            println!("Username: {}\nEmail: {}\nActive: {}\nCount: {}", self.username, self.email, self.active, self.count);
        }
    }

    fn change_username(user: &mut User, new_username:&str){
        user.username = String::from(new_username);
    }

    #[test]
    fn tests_structs(){
        let mut user1 = User::new_user();
        user1.increment_count(1);
        user1.increment_count(2);
        user1.show_info();

        change_username(&mut user1, "Bulan");
    }
}