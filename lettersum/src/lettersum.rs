pub fn lettersum(string: &str) -> i32 {
    let mut sum  = 0;
    
    for letter in string.chars() {
        sum += letter as i32 - 96;
    }

    return sum;
}