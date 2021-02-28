
/**
 * [205] Isomorphic Strings
 *
 * Given two strings s and t, determine if they are isomorphic.
 * Two strings s and t are isomorphic if the characters in s can be replaced to get t.
 * All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
 *  
 * Example 1:
 * Input: s = "egg", t = "add"
 * Output: true
 * Example 2:
 * Input: s = "foo", t = "bar"
 * Output: false
 * Example 3:
 * Input: s = "paper", t = "title"
 * Output: true
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 5 * 10^4
 * 	t.length == s.length
 * 	s and t consist of any valid ascii character.
 * 
 */
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let matrix = s.into_bytes().into_iter().zip(t.into_bytes());
        let (mut s_c_vev, mut t_c_vec) = (vec![0;256], vec![0;256]);
        for (i,(s_c,t_c)) in matrix.enumerate() {
            if s_c_vev[s_c as usize] != t_c_vec[t_c as usize] {
                return false;
            }
            s_c_vev[s_c as usize] = i + 1;
            t_c_vec[t_c as usize] = i + 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_205() {
        debug_assert_eq!(Solution::is_isomorphic("egg".to_owned(), "app".to_owned()), true);
        debug_assert_eq!(Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()), false);
        debug_assert_eq!(Solution::is_isomorphic("paper".to_owned(), "title".to_owned()), true);
    }
}