#![allow(non_snake_case)]
#![allow(unused_must_use)]
#[macro_use] extern crate text_io;

fn sum_of_digits<'a>(n: i32) -> &'a str {
    // return "YES" if n = 1 or n = 4
    if n == 1 || n == 4 {
        return "YES\n";
    }
    // return "NO" when n = {9 or 16}, causes infty loop
    else if n == 9 || n == 16 {
        return "NO\n";
    }
    // every other number
    else {
        let num = n*n;
        // sum of digits of n, eg. 65 = 6 + 5 = 11
        let mut sum = 0;
        let mut i = num;

        while i != 0 {
            sum += i % 10;      // 65 % 10 = 5
            i /= 10;            // 65 / 10 = 6
        }

        // recurrency for new n = 11
        return sum_of_digits(sum);
    }
}

fn main() {
    // read line and cast it to fire... i32
    print!("T: ");
    let T: i32 = read!();

    // read tests and check the answers
    for i in 0..T {
        print!("N_{}: ", i+1);
        let N: i32 = read!();
        print!("{}", sum_of_digits(N));
    }
}
