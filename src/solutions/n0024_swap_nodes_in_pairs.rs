/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [2,1,4,3]
 * 
 * Example 2:
 * 
 * Input: head = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: head = [1]
 * Output: [1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [0, 100].
 * 	0 <= Node.val <= 100
 * 
 *  
 * Follow up: Can you solve the problem without modifying the values in the list's nodes? (i.e., Only nodes themselves may be changed.)
 */
pub struct Solution {}
#[allow(unused_imports)]
use crate::models::linked_list::{ListNode, to_list};

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
#[allow(dead_code)]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut prev = head.as_mut();
        while let Some(node) = prev {
            if node.next.is_none() || node.next.as_ref().unwrap().next.is_none() { break; }
            let mut cur = node.next.take();
            let mut next = cur.as_mut().unwrap().next.take();
            let next_next = next.as_mut().unwrap().next.take();
            node.next = next;
            node.next.as_mut().unwrap().next = cur;
            node.next.as_mut().unwrap().next.as_mut().unwrap().next = next_next;
            prev = node.next.as_mut().unwrap().next.as_mut();
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(Solution::swap_pairs(to_list(vec![1, 2, 3, 4])), to_list(vec![2, 1, 4, 3]));
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
        assert_eq!(Solution::swap_pairs(to_list(vec![1, 2, 3])), to_list(vec![2, 1, 3]));
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}