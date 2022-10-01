#![warn(dead_code)]

/*
    *** Basic formatting ***

    # Placeholders
    # Positional arguments
    # Named arguments
    # Placeholder traits
    # Basic math
    # Debug trait will be used later
*/

fn main() {
    /* # Placeholders
                           placeholder notation
                           v
        println!("Pi value {}", 3.14);
                                ^
                                placeholder value
     */
    println!("Pi value: {}\n", 3.14);

    /*
    You can use multiple placeholders, but every one must have its value
                  1.       2.   1.      2.
                  v        v    v       v
        println!("{} is my {}", "This", "Tutorial");
    */
    println!("{} is my {}\n", "This", "Tutorial");

    /* # Positional arguments
                                "Rust"   "Rust"      "easy"
                                v        v           v      
        println!("You can learn {0} too. {0} is very {1}!", "Rust", "easy");
    */
    println!("You can learn {0} too. {0} is very {1}!\n", "Rust", "easy");

    /* # Named arguments
                      "Bond"   "James"  "Bond"
                      v          v      v
        println!("I'm {surname}, {name} {surname}", name="James", surname="Bond");
    */
    println!("I'm {surname}, {name} {surname}\n", name="James", surname="Bond");

    /* # Placeholder traits
        You can convert values to:
            - {:b} - binary
            - {:x} - hexadecimal
            - {:o} - octal
    */
    println!("Number: {0} in base of:\n- 2: {0:b}\n- 16: {0:x}\n- 8: {0:o}\n", 31);

    /* # Basic math
        You can print simple math
    */
    println!("{} + {} = {}\n", 5, 3, 5+3);

    /* # Debug trait
        Print structures etc.
    */
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