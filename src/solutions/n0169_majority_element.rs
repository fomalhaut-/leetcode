/**
 * [169] Majority Element
 *
 * Given an array nums of size n, return the majority element.
 * The majority element is the element that appears more than &lfloor;n / 2&rfloor; times. You may assume that the majority element always exists in the array.
 *  
 * Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 5 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 *  
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 */
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for &num in nums.iter() {
            if count == 0 {
                candidate = num;
            }
            count += if num == candidate { 1 } else { -1 };
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_169() {
        assert_eq!(Solution::majority_element(vec![3,2,3]), 3);
        assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
    }
}