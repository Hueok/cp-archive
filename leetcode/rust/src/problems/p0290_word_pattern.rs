pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s: Vec<_> = s.split_whitespace().collect();
        let p = pattern.as_bytes();

        let mut bijection: [String; 26] = std::array::from_fn(|_| String::new());

        for i in 0..pattern.len(){
            if bijection[(p[i]-b'a') as usize].len() == 0 {
                // first matching
                bijection[(p[i]-b'a') as usize].push_str(s[i]);
            } else{
                // already mapped
                if bijection[(p[i]-b'a') as usize] != s[i] {
                    return false;
                }
            }
        }
        true
        //write solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_2() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }
    #[test]
    fn test_case_3() {
        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        assert_eq!(result, false);
    }
}