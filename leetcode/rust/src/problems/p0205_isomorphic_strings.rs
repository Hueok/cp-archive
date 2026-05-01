pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let mut dict = HashMap::new();
        let mut used = [false; 129];

        for i in 0..s.len() {
            if let Some(v) = dict.get_mut(&s[i]) {
                // s -> t character already used
                if *v != t[i] {
                    return false;
                }
            } else{
                // s -> t first time mapping
                if used[t[i] as usize] {
                    // t domain already mapped
                    return false;
                }
                dict.insert(s[i], t[i]);
                used[t[i] as usize] = true;
            }
        }
        return true;

    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("egg");
        let t = String::from("add");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_2() {
        let s = String::from("f11");
        let t = String::from("b23");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, false);
    }
    #[test]
    fn test_case_3() {
        let s = String::from("paper");
        let t = String::from("title");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }
}