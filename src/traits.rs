pub trait Animal {
    fn make_sound(&self) -> ();
}

/* //////////////////////////////////////////////////////////////
                    Associating traits with structs
////////////////////////////////////////////////////////////// */
pub struct Dog {}
pub struct Cat {}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Dog 🐶 says Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Cat 😺 says Meow!");
    }
}

pub struct Person<Pet: Animal> {
    pub pet: Pet,
}

pub fn create_person(_pet: impl Animal) -> Person<impl Animal> {
    return Person { pet: _pet };
}
