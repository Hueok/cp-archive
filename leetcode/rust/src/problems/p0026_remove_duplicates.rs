pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i: usize = 0;
        for j in 1..nums.len(){
            if nums[i] < nums[j] {
                i+=1;
                nums[i] = nums[j];
            }
        }
        i as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums = vec![1,1,2];
        let result = Solution::remove_duplicates(&mut nums);
        let arr = vec![1,2];
        assert_eq!(result, 2);
        for i in 0..arr.len(){
            assert_eq!(nums[i as usize], arr[i as usize]);
        }
    }
    #[test]
    fn test_case_2() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        let result = Solution::remove_duplicates(&mut nums);
        let arr = vec![0,1,2,3,4];
        assert_eq!(result, arr.len() as i32);
        for i in 0..arr.len(){
            assert_eq!(nums[i as usize], arr[i as usize]);
        }
    }
}