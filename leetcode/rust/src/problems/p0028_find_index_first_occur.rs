pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let result = Self::kmp_search(&haystack, &needle);
        if result.len() > 0 {
            result[0] as i32
        } else {
            -1
        }
    }
    pub fn build_lps(pattern: &str) -> Vec<usize> {
        let p = pattern.as_bytes();
        let mut lps = vec![0; p.len()];

        let mut len = 0;
        let mut i = 1;

        while i<p.len() {
            if p[i] == p[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];

                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        lps
    }
    pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
        let t = text.as_bytes();
        let p = pattern.as_bytes();

        let lps = Self::build_lps(pattern);

        let mut result = Vec::new();

        let (mut i, mut j) = (0, 0);
        // i -> text index
        // j -> pattern index

        while i < t.len() {
            if t[i] == p[j] {
                i += 1;
                j += 1;
            }

            if j == p.len() {
                result.push(i-j);

                j = lps[j-1];
            } else if i < t.len() && t[i] != p[j] {
                if j != 0{
                    j = lps[j-1];
                } else {
                    i += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, 0);
    }
    #[test]
    fn test_case_2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, -1);
    }
}