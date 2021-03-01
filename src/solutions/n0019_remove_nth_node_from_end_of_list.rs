/**
 * [19] Remove Nth Node From End of List
 *
 * Given the head of a linked list, remove the n^th node from the end of the list and return its head.
 * Follow up: Could you do this in one pass?
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], n = 2
 * Output: [1,2,3,5]
 * 
 * Example 2:
 * 
 * Input: head = [1], n = 1
 * Output: []
 * 
 * Example 3:
 * 
 * Input: head = [1,2], n = 1
 * Output: [1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is sz.
 * 	1 <= sz <= 30
 * 	0 <= Node.val <= 100
 * 	1 <= n <= sz
 * 
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (mut head, mut n) = (head, n as u32);
        let mut p1 = &mut head as *mut Option<Box<ListNode>>;
        let mut p2 = &mut head as *mut Option<Box<ListNode>>;
        unsafe {
            while (*p1).is_some() {
                p1 = &mut (*p1).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                if n == 0 { p2 = &mut (*p2).as_mut().unwrap().next as *mut Option<Box<ListNode>>; }
                n = n.saturating_sub(1);
            }
            if (*p2).is_some() { p2.replace((*p2).as_mut().unwrap().next.take()); }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1,2,3,4,5]), 2),
                   to_list(vec![1,2,3,5]));
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1),
                   None);
    }
}