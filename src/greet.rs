use std::fmt::Debug;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub sex: String,
    pub race: String,
    pub age: u8,
}

impl User {
    pub fn generate(name: &str, sex: &str, race: &str, age: u8) -> User {
        User {
            name: String::from(name),
            sex: String::from(sex),
            race: String::from(race),
            age: age,
        }
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}
