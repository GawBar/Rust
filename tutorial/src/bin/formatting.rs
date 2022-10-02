/////////////////////////////// Basic formatting ///////////////////////////////
// 
// # Placeholders
// # Positional arguments
// # Named arguments
// # Placeholder traits
// # Basic math
// # Debug trait will be used later
//
fn main() {
    // Placeholders
    //
    // You can print data using placeholder ( {} )
    // 
    println!("Pi value: {}\n", 3.14);
    println!("{} is my {}\n", "This", "Tutorial");

    // Positional arguments
    //
    // Positional placeholders are counted from 0:
    //  {0} - placeholder for first argument
    //  {1} - placeholder for second argument
    //  ...
    //
    println!("You can learn {0} too. {0} is very {1}!\n", "Rust", "easy");

    // Named arguments
    //
    // You can name your arguments and use them in placeholders
    //
    println!("I'm {surname}, {name} {surname}\n", name="James", surname="Bond");

    // Placeholder traits
    //
    // You can convert values to:
    //  - {:b} - binary
    //  - {:x} - hexadecimal
    //  - {:o} - octal
    //
    println!("Number: {0} in base of:\n- 2: {0:b}\n- 16: {0:x}\n- 8: {0:o}\n", 31);

    // Basic math
    println!("{} + {} = {}\n", 5, 3, 5+3);
    println!("{1} - {2} = {0}\n", 7-4, 7, 4);

    // Debug trait
    // 
    // Print structures etc.
    //
    #[derive(Debug)]
    struct Person<'a> {
        _name: &'a str,
        _age: u8
    }

    let name = "Harry";
    let age = 19;
    let harry = Person { _name: name, _age: age };

    // Pretty print
    println!("{:#?}", harry);
}