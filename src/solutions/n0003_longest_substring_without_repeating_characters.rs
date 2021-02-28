/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest substring without repeating characters.
 *  
 * Example 1:
 * 
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 * 
 * Example 2:
 * 
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 * 
 * Example 3:
 * 
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 * 
 * Example 4:
 * 
 * Input: s = ""
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 * 
 */
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut i, mut j, mut ans) = (0, 0, 0);
        let mut set = std::collections::hash_map::HashMap::new();
        let chars = s.as_bytes();
        while j < s.len() {
            if set.contains_key(&chars[j]) {
                set.remove(&chars[i]);
                i += 1;
            }else{
                set.insert(chars[j], 0);
                j += 1;
            }
            ans = std::cmp::max(ans, set.len());
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_owned()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_owned()), 3);
        assert_eq!(Solution::length_of_longest_substring("".to_owned()), 0);
    }
}