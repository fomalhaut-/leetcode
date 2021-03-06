/**
 * [111] Minimum Depth of Binary Tree
 *
 * Given a binary tree, find its minimum depth.
 * The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
 * Note: A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex_depth.jpg" style="width: 432px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 2
 * 
 * Example 2:
 * 
 * Input: root = [2,null,3,null,4,null,5,null,6]
 * Output: 5
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 10^5].
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
#[allow(unused_imports)]
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => return 0,
            Some(node) => {
                let mut min_depth = i32::max_value();
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if left.is_none() && right.is_none() {
                    return 1;
                }
                if left.is_some() {
                    min_depth = std::cmp::min(Solution::min_depth(left), min_depth);
                }
                if right.is_some() {
                    min_depth = std::cmp::min(Solution::min_depth(right), min_depth);
                }
                min_depth + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111() {
        assert_eq!(Solution::min_depth(tree![3,9,20,null,null,15,7]), 2);
        assert_eq!(Solution::min_depth(tree![2,null,3,null,4,null,5,null,6]), 5);
        assert_eq!(Solution::min_depth(tree![1,2,3,4,5]), 2);
       // println!("{:?}", tree![1,2,3,4,5].unwrap().borrow());
    }
}