enum Animal{
    Dog,
    Cat
}
#[derive(Debug)]
enum Role {
    Attacker,
    Healer,
}

struct Hero {
    name: &'static str,
    role: Role
}

trait Doing {
    fn speak(&self);
}

impl Doing for Animal {
    fn speak(&self){
        match &self {
            Animal::Dog => println!("woof"),
            Animal::Cat => println!("meow")
        }
    }
}

impl Doing for Hero {
    fn speak(&self) {
        println!("I am a {:?}. My name is {}", &self.role, &self.name);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits(){
        let dog1 = Animal::Dog;
        dog1.speak();

        let hero1 = Hero{
            name: "Gabe",
            role: Role::Attacker
        };

        hero1.speak();
    }
}