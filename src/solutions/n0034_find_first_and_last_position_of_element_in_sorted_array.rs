/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 * If target is not found in the array, return [-1, -1].
 * Follow up: Could you write an algorithm with O(log n) runtime complexity?
 *  
 * Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * Example 3:
 * Input: nums = [], target = 0
 * Output: [-1,-1]
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	nums is a non-decreasing array.
 * 	-10^9 <= target <= 10^9
 * 
 */
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() { return vec![-1, -1]; }
        fn search_bound(nums: &Vec<i32>, target: i32) -> i32 {
            let (mut left, mut right) = (0, (nums.len() - 1) as i32);
            let mut ans = nums.len() as i32;
            while left <= right {
                let middle = (left + right) / 2;
                if nums[middle as usize] >= target {
                    right = middle - 1;
                    ans = middle;
                } else {
                    left = middle + 1;
                }
            }
            return ans;
        }
        let left = search_bound(&nums, target);
        let right = search_bound(&nums, target + 1) - 1;

        return if left <= right && right < nums.len() as i32 && nums[left as usize] == target && nums[right as usize] == target {
            vec![left as i32, right as i32]
        } else {
            vec![-1, -1]
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(Solution::search_range(vec![2,2], 1), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
    }
}