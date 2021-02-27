/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is height-balanced.
 * 
 * For this problem, a height-balanced binary tree is defined as:
 * 
 * <blockquote>
 * a binary tree in which the left and right subtrees of every node differ in height by no more than 1.
 * </blockquote>
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg" style="width: 342px; height: 221px;" />
 * 
 * Input: root = [3,9,20,null,null,15,7]
 * Output: true
 * 
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg" style="width: 452px; height: 301px;" />
 * 
 * Input: root = [1,2,2,3,3,null,null,4,4]
 * Output: false
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: root = []
 * Output: true
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-10^4 <= Node.val <= 10^4
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            return if let Some(node) = root {
                let p = (height(node.borrow().left.clone()), height(node.borrow().right.clone()));
                match p {
                    (Some(left), Some(right)) => {
                        if (left - right).abs() < 2 {
                            Some(i32::max(left, right) + 1)
                        }else{
                            None
                        }
                    },
                    _ => None,
                }
            } else {
                Some(0)
            }
        }

        height(root).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        assert_eq!(Solution::is_balanced(tree![3,9,20,null,null,15,7]), true);
        assert_eq!(Solution::is_balanced(tree![]), true);
    }
}