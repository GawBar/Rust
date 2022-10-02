//! An explanation of rustdoc on simple example with structs
//! 
//! Created some animals
//!
//
// Generation of doc:
//  > cargo doc --open
//
/// Every animal says something, didn't it?
/// Shared behavior for all animals
trait Say {
    fn say(&self);
}

/// Animal being is represented here
struct Animal {
    /// Animal have its age
    _age: u8
}

/// Cat represented is here, it purrs... awww
struct Cat {
    /// Cat inherits age from Animal
    _animal: Animal,
    /// Cat have name, no matter if never responds on it when called
    _name: String
}

/// Woof! Woof!
struct Dog {
    /// Wrrrr!
    _animal: Animal,
    /// Arf!
    _name: String
}

/// Implementation of Animal
impl Animal {
    /// Animal is born!
    fn new(age: u8) -> Self {
        Animal { _age: age }
    }
}

/// Kitty, kitty, kitty!
impl Cat {
    fn new(name: String, age: u8) -> Self {
        Cat { _name: name, _animal: Animal::new(age) }
    }
}

/// The bowl is empty human...
impl Say for Cat {
    fn say(&self) {
        println!("Meow!");
    }
}

/// Yaaawn!
impl Dog {
    /// Barf?
    fn new(name: String, age: u8) -> Self {
        Dog { _name: name, _animal: Animal::new(age) }
    }
}

/// Throw the ball! NOW!
impl Say for Dog {
    fn say(&self) {
        println!("Woof!");
    }
}

fn main() {
    // some dog
    let dog = Dog::new("Ralph".to_string(), 7);
    // some cat
    let cat = Cat::new("Kitty".to_string(), 4);

    // say!
    dog.say();
    cat.say();
}