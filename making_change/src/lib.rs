mod change;

#[cfg(test)]
mod tests {
    use super::*;
    use change::change;

    #[test]
    fn it_works() {
        assert_eq!(change(0), 0);
        assert_eq!(change(12), 3);
        assert_eq!(change(468), 11);
        assert_eq!(change(123456), 254);
    }
}
