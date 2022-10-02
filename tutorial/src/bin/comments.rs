/////////////////////////////////// Comments ///////////////////////////////////
// 
// There are comments like:
//  // ...      - line comment
//  /* ... */   - block comment
//
//  /// ...     - doc line comment
//  /** ... */  - doc block comment
//
//  //! ...     - inner doc comment
//  /*! ... */  - inner doc block comment
//
// You can use Markdown notation in doc
//

//! Hello World program

/// This function returns the greeting: Hello, world!
pub fn hello() -> String {
    ("Hello, world!").to_string()
}

fn main() {
    println!("{}", hello());
}