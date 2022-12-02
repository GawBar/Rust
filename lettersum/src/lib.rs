#[allow(dead_code)]

fn lettersum(string: &str) -> i32 {
    let mut sum  = 0;
    
    for letter in string.chars() {
        sum += letter as i32 - 96;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(lettersum(""), 0);
        assert_eq!(lettersum("a"), 1);
        assert_eq!(lettersum("z"), 26);
        assert_eq!(lettersum("cab"), 6);
        assert_eq!(lettersum("excellent"), 100);
        assert_eq!(lettersum("microspectrophotometries"), 317);
    }
}
