#[allow(dead_code)]
#[allow(unused_imports)]

mod lettersum;

#[cfg(test)]
mod tests {
    use super::*;
    use lettersum::lettersum;

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
