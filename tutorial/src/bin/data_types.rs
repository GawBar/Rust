////////////////////////////////// Data types //////////////////////////////////
// 
// Variables can be defined in two ways:
//  - implicit definition
//  - explicit definition
//
// Primitive types in Rust:
//  - scalar
//      - numeric
//          - integer
//              - signed: i8, i16, i32, i64
//              - unsigned: u8, u16, u32, u64
//              - variable size, defined by architecture: isize, usize
//          - float
//              - f32, f64
//      - non numeric
//          - boolean
//              - true
//              - false
//          - characters
//              - char
//              - &str
//  - compound
//      - array
//      - tuple
// 


fn main() {
    // implicit definition, Rust detects type of variable
    // some IDE shows what type is used, in this example it is i32
    let implicit_var = 23;
    println!("Implicit variable: {}\n", implicit_var);

    // explicit definition, tell the type of variable
    let explicit_var:f32 = 3.1415;
    println!("Explicit variable: {}\n", explicit_var);

    // boolean
    let is_23_lt_2: bool = 23 < 2;
    println!("23 < 2: {}\n", is_23_lt_2);

    // character
    let char_var: char = 'U';
    println!("Character: {}\n", char_var);

    // string
    let str_var: &str = "Rust";
    println!("String: {}\n", str_var);

    // Array is homogenous sequence of elements
    //
    // definition:
    //  let name: [type; size] = [el1, el2, ...];
    //
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}\n", array);

    // You can access elements of array, count elements from 0
    println!("Second element of array: {}\n", array[1]);

    // Arrays as variables can be mutable
    let mut mut_array: [i32; 3] = [2, 6, 8];
    println!("mut_array: {:?}", mut_array);
    mut_array[1] = 1;
    println!("mut_array: {:?}\n", mut_array);

    // length of array
    println!("Length of array: {}", array.len());
    println!("Length of mut_array: {}\n", mut_array.len());

    // Slices
    //
    // Size of slices is not known at complite time so you must use pointer to
    // array slice, starting index is inclusive, ending index is exclusive
    //
    //  array[x..y]     from x index to y index
    //  array[..y]      from beginning to y index
    //  array[x..]      from x index to end
    //  array[..]       full array
    //
    let new_array: [i32; 10] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 45];
    println!("new_rray from:");
    println!("  beginning to 4th index: {:?}", &new_array[..4]);
    println!("  2nd to 9th index: {:?}\n", &new_array[2..9]);

    // Tuples are heterogeneous sequences of elements, each element can have
    // different type
    //
    let bond = ("Bond", 007, "Mi6");
    
    // access by dot operator ( . )
    println!("By . operator:\n  Agent {}, name {}, works for {}", bond.1, bond.0, bond.2);
    
    // access by pattern
    let (name, number, agency) = bond;
    println!("By pattern:\n  Agent {}, name {}, works for {}", number, name, agency);
}