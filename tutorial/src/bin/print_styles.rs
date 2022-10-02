//////////////////////////////// Printing styles ///////////////////////////////
// 
// There are macros:
//  print! - prints string to screen
//  println! - same as print! but adds new line on end
//  eprint! - prints string to screen as error
//  eprintln! - same as eprint! but adds new line on end
//
fn main() {
    // print!
    print!("This is ");
    print!("my Rust tutorial\n");

    // println!
    println!("This is my Rust tutorial");

    // eprint!
    print!("This is ");
    print!("my error?\n");

    // eprintln!
    eprintln!("This is my error?");
}