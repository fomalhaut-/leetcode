/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest palindromic substring in s.
 * 
 *  
 * Example 1:
 * 
 * 
 * Input: s = "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "cbbd"
 * Output: "bb"
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: s = "a"
 * Output: "a"
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: s = "ac"
 * Output: "a"
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters (lower-case and/or upper-case),
 * 
 */
pub struct Solution {}

/**
public String longestPalindrome(String s) {
        if (s == null || s.length() < 1) {
            return "";
        }
        int start = 0, end = 0;
        for (int i = 0; i < s.length(); i++) {
            int len1 = expandAroundCenter(s, i, i);
            int len2 = expandAroundCenter(s, i, i + 1);
            int len = Math.max(len1, len2);
            if (len > end - start) {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }
        return s.substring(start, end + 1);
    }
**/
#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn expand(s:&String, mut left:i32, mut right:i32) -> i32{
            let chars:Vec<char> = s.chars().collect();
            while left >= 0 && right < s.len() as i32 && chars[left as usize] == chars[right as usize]{
                left -= 1;
                right += 1;
            }
            right - left -1
        }
        let s_len = s.len();
        if s_len < 1{
            return "".to_owned();
        }

        let (mut start, mut end) = (0usize, 0usize);
        for i in 0..(s_len as i32) {
            let len1 = expand(&s,i as i32,i as i32);
            let len2 = expand(&s, i as i32, (i + 1) as i32);
            let len = std::cmp::max(len1, len2);
            if len > (end - start) as i32 {
                start = (i - (len - 1) / 2)  as usize;
                end = (i + len / 2) as usize;
            }
        }

        return s[start..(end+1)].to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("babad".to_owned()), "aba");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
    }
}