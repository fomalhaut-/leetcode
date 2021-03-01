/**
 * [203] Remove Linked List Elements
 *
 * Remove all elements from a linked list of integers that have value val.
 * 
 * Example:
 * 
 * 
 * Input:  1->2->6->3->4->5->6, val = 6
 * Output: 1->2->3->4->5
 * 
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val != val {
                next.as_mut().unwrap().next = Some(node);
                next = next.unwrap().next.as_mut();
            }
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_203() {
        assert_eq!(Solution::remove_elements(linked![1,2,6,3,4,5,6], 6), linked![1,2,3,4,5]);
    }
}