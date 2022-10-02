///////////////////////////// Scope and shadowing //////////////////////////////
// 
// Scope of a variable means, that variable has visibility and access to parts 
// of program. It depends where variable is declared
//
// There are two types ov variables:
//  - global variables
//  - local variables
//
fn main() {
    // outer scope
    let global_var = 991;
    // block of code is inner scope
    {
        // global_var is visible in this scope
        let local_var = 112;
        println!("Global variable: {}", global_var);
        // local_var also is visible
        println!("Local variable: {}\n", local_var);
    }
    // local_var isn't visible in this scope :(
    // println!("Local variable: {}", local_var);

    // to fix this error we can use
    let global_var1 = 666;
    let local_var1 = 777;
    {
        // global_var is visible in this scope
        println!("Global variable: {}", global_var1);
        // local_var is visible in this scope, is global now
        println!("Local variable: {}", local_var1);
    }
    // Now we can acces local_var, it's visible in this scope
    println!("Local variable: {}\n", local_var1);

    // Shadowing
    //
    // This is technique of using same variable name in outer and inner scope,
    // outer variable is shadowed by inner variable, while inner variable is
    // mask of outer variable
    //
    let outer_variable = 123;
    println!("OUTER SCOPE:");
    println!("outer_variable: {}", outer_variable);
    {
        let inner_variable = 456;
        let outer_variable = 789;

        println!("  INNER SCOPE:");
        println!("  outer_variable: {}", outer_variable);
        println!("  inner_variable: {}", inner_variable);
    }
    println!("outer_variable: {}", outer_variable);
}