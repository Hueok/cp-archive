pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let input: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        (0..=input.len()/2 - 1).all(|i| input[i] == input[input.len()-1-i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = String::from("A man, a plan, a canal: Panama");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_2() {
        let input = String::from("race a car");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, false);
    }
    #[test]
    fn test_case_3() {
        let input = String::from(" ");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, true);
    }
}