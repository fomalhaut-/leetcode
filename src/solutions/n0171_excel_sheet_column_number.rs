/**
 * [171] Excel Sheet Column Number
 *
 * Given a column title as appear in an Excel sheet, return its corresponding column number.
 * 
 * For example:
 * 
 * 
 *     A -> 1
 *     B -> 2
 *     C -> 3
 *     ...
 *     Z -> 26
 *     AA -> 27
 *     AB -> 28 
 *     ...
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: "A"
 * Output: 1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "AB"
 * Output: 28
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "ZY"
 * Output: 701
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 7
 * 	s consists only of uppercase English letters.
 * 	s is between "A" and "FXSHRXW".
 * 
 */
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn title_to_number(mut s: String) -> i32 {
        let mut res = 0;
        let mut i = 0;
        while let Some(ch) = s.pop() {
            res += 26i32.pow(i) * (ch as u8 - 64) as i32;
            i += 1;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_171() {
        assert_eq!(Solution::title_to_number("A".parse().unwrap()), 1);
        assert_eq!(Solution::title_to_number("AB".parse().unwrap()), 28);
        assert_eq!(Solution::title_to_number("ZY".parse().unwrap()), 701);
    }
}