mod problems;

#[cfg(test)]
mod tests {
    use super::problems::p0001_two_sum::Solution;

    #[test]
    fn test_two_sum() {
        let result = Solution::two_sum(vec![2,7,11,15], 9);
        assert_eq!(result, vec![0,1]);
    }
}
