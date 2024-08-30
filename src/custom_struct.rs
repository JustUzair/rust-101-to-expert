pub struct User {
    pub name: String,
    pub age: u8,
    pub email: String,
}

#[derive(Debug)]
enum VehicleColor {
    Red,
    Blue,
    Green,
    Black,
    White,
}

#[derive(Debug)]
pub struct Vehicle {
    color: VehicleColor,
    model: String,
    company: String,
}

// Tuple Struct
struct TupleUser(String, u8, String);

pub fn create_user(name: String, age: u8, email: String) -> User {
    return User { name, age, email };
}

impl User {
    pub fn print_user(&self) {
        println!("User Name: {:?}", self.name);
        println!("User Age: {:?}", self.age);
        println!("User Email: {:?}", self.email);
    }
}
