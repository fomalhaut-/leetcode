/**
 * [11] Container With Most Water
 *
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of the line i is at (i, ai) and (i, 0). Find two lines, which, together with the x-axis forms a container, such that the container contains the most water.
 * Notice that you may not slant the container.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg" style="width: 600px; height: 287px;" />
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
 * 
 * Example 2:
 * 
 * Input: height = [1,1]
 * Output: 1
 * 
 * Example 3:
 * 
 * Input: height = [4,3,2,1,4]
 * Output: 16
 * 
 * Example 4:
 * 
 * Input: height = [1,2,1]
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	n == height.length
 * 	2 <= n <= 3 * 10^4
 * 	0 <= height[i] <= 3 * 10^4
 * 
 */
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut i, mut j) = (0, height.len() - 1);
        while i < j {
            let area = std::cmp::min(height[i], height[j]) * (j as i32 - i as i32);
            max = std::cmp::max(max, area);
            if height[i] > height[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(Solution::max_area(vec![1,1]), 1);
        assert_eq!(Solution::max_area(vec![4,3,2,1,4]), 16);
        assert_eq!(Solution::max_area(vec![1,2,1]), 2);
    }
}