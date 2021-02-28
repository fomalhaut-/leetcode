/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 *  
 * Example 1:
 * 
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 * 
 * Example 2:
 * 
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 * 
 * Example 3:
 * 
 * Input: nums1 = [0,0], nums2 = [0,0]
 * Output: 0.00000
 * 
 * Example 4:
 * 
 * Input: nums1 = [], nums2 = [1]
 * Output: 1.00000
 * 
 * Example 5:
 * 
 * Input: nums1 = [2], nums2 = []
 * Output: 2.00000
 * 
 *  
 * Constraints:
 * 
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 * 
 *  
 * Follow up: The overall run time complexity should be O(log (m+n)).
 */
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        return if nums1.len() > nums2.len() {
            Solution::find_median(&nums2,&nums1)
        } else {
            Solution::find_median(&nums1,&nums2)
        }
    }
    pub fn find_median(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        let (mut i_min, mut i_max) = (0, m);
        while i_min <= i_max {
            let i = (i_min + i_max) / 2;
            let j = (m + n + 1) / 2 - i;
            if j != 0 && i != m && nums2[j - 1] > nums1[i] {
                i_min = i + 1;
            } else if i != 0 && j != n && nums1[i - 1] > nums2[j] {
                i_max = i - 1;
            } else {
                let max_left = if i == 0 {
                    nums2[j - 1]
                }else if j == 0 {
                    nums1[i - 1]
                }else{
                    std::cmp::max(nums1[i-1], nums2[j-1])
                };
                if (m + n) % 2 == 1 { return max_left as f64; }

                let min_right = if i == m {
                    nums2[j]
                }else if j == n {
                    nums1[i]
                }else{
                    std::cmp::min(nums1[i], nums2[j])
                };
                return (max_left + min_right) as f64 / 2.0;
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_4() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2f64);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![0,0], vec![0,0]), 0f64);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1f64);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2f64);
    }
}