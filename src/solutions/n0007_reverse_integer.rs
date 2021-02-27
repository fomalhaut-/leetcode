/**
 * [7] Reverse Integer
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *  
 * Example 1:
 * Input: x = 123
 * Output: 321
 * Example 2:
 * Input: x = -123
 * Output: -321
 * Example 3:
 * Input: x = 120
 * Output: 21
 * Example 4:
 * Input: x = 0
 * Output: 0
 *  
 * Constraints:
 * 
 * 	-2^31 <= x <= 2^31 - 1
 * 
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev = 0;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            if rev > i32::MAX /10 || rev == i32::MAX / 10 && pop > 7 {return 0};
            if rev < i32::MIN /10 || rev == i32::MIN / 10 && pop < -8 {return 0};
            rev = rev * 10 + pop;
        }
        return rev;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_7() {
        let expect = Solution::reverse(123);
        assert_eq!(expect, 321);
    }
}