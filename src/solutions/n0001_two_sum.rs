/**
 * [1] Two Sum
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * You can return the answer in any order.
 *  
 * Example 1:
 * 
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Output: Because nums[0] + nums[1] == 9, we return [0, 1].
 * 
 * Example 2:
 * 
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 * 
 * Example 3:
 * 
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 10^3
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 	Only one valid answer exists.
 * 
 */
pub struct Solution {}
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, x) in nums.into_iter().enumerate(){
            if let Some(&j) = map.get(&(target - x)) {
                return vec![j as i32, i as i32];
            }else{
                map.insert(x, i);
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0,1]);
    }
}