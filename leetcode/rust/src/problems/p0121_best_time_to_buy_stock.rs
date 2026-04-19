pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        // write solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let prices = vec![7,1,5,3,6,4];
        let result = Solution::max_profit();
        assert_eq!(result, 5);
    }
}