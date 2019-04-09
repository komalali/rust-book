mod sound;

// If you use pub before a struct definition, you make the struct public. However, the struct's fields are still private. You can choose to make each field public or not.
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

// If you make a public enum, all of its variants are public.
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();

    sound::instrument::clarinet();

    let mut v = plant::Vegetable::new("Cucumber");
    v.name = String::from("Persian cucumbers");
    println!("{} are delicious", v.name);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    // The next line would not compile if I uncomment it because id is not a public field.
    // println!("The ID is {}", v.id);
}
