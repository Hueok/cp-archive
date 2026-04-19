pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut arr: Vec<i32> = nums.clone();
        arr.sort_unstable();
        arr[arr.len()/2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![3,2,3];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_case_2() {
        let nums = vec![2,2,1,1,1,2,2];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 2);
    }
}