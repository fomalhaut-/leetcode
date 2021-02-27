/**
 * [112] Path Sum
 *
 * Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
 * A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum1.jpg" style="width: 500px; height: 356px;" />
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg" />
 * Input: root = [1,2,3], targetSum = 5
 * Output: false
 * 
 * Example 3:
 * 
 * Input: root = [1,2], targetSum = 0
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-1000 <= Node.val <= 1000
 * 	-1000 <= targetSum <= 1000
 * 
 */
pub struct Solution {}
use crate::models::tree::{TreeNode, to_tree};

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        return match root {
            None => false,
            Some(root) => {
                let left = root.borrow().left.clone();
                let right = root.borrow().right.clone();
                if left.is_none() && right.is_none() {
                    return target_sum == root.borrow().val;
                }
                let remain_sum = target_sum - root.borrow().val;
                Solution::has_path_sum(left, remain_sum) || Solution::has_path_sum(right, remain_sum)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
        assert_eq!(Solution::has_path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,null,1], 22), true);
        assert_eq!(Solution::has_path_sum(tree![1,2,3], 5), false);
        assert_eq!(Solution::has_path_sum(tree![1,2], 0), false);
    }
}