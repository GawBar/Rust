mod flipfront;

#[cfg(test)]
mod tests {
    use super::*;
    use flipfront::flipfront;
    
    #[test]
    fn it_works() {
        assert_eq!(flipfront(vec![0, 1, 2, 3, 4], 2), [1, 0, 2, 3, 4]);
        assert_eq!(flipfront(vec![0, 1, 2, 3, 4], 3), [2, 1, 0, 3, 4]);
        assert_eq!(flipfront(vec![0, 1, 2, 3, 4], 5), [4, 3, 2, 1, 0]);
        assert_eq!(flipfront(vec![1, 2, 2, 2], 3), [2, 2, 1, 2]);
    }
}
