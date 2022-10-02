/////////////////////////////////// Variables //////////////////////////////////
//
// # Create variable
// # Initialize variable
// # Mutability
// # Assigning multiple variables
//
fn main() {
    // Create variable
    //
    // By convention, you should write a variable name in a snake_case i.e.,
    // - all letters should be lower case.
    // - all words should be separated using an underscore ( _ ).
    //
    // Initialized variable must have its type, there are many types i.e.
    // - integer
    // - floating point
    // - boolean
    // - string
    // - ...
    //
    // If you want to keep variable un-assigned or unused you can use underscore
    // ( _ ) in the beginning of variable name
    //
    let _var_int: i32;      // integer type
    let _var_string: &str;  // string type
    let _var_float: f64;    // float type
    let _var_bool: bool;    // boolean type
    let _var_char: char;    // character type

    // Initialize variable
    //
    // Rust detects the type of the variable so you don't have to define the type
    //
    let variable = "Rust";  // string type: &str
    println!("Tutorial in {}\n", variable);

    // Mutability
    //
    // By default all variables are immutable, you can initialize it once in code.
    // If you want to use variable multiple times in code you should use it as 
    // mutable
    //
    let mut mut_var = "Rust";
    println!("I'm programming in {}", mut_var);
    mut_var = "Java";
    println!("I'm programming in {} too\n", mut_var);

    // Assign multiple variables
    //
    // You can assign multiple variables by using () or []
    //
    let [lang, level] = ["Rust", "begginers"];
    println!("This is {} tutorial for {}", lang, level);

    let (name, surname) = ("James", "Bond");
    println!("My name is {1}. {0} {1}", name, surname);
}