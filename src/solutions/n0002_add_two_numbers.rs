/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * Example 2:
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * Example 3:
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 * 
 */
pub struct Solution {}
use crate::models::linked_list::{ListNode, to_list};

#[allow(dead_code)]
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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let (mut l1, mut l2, mut carry, mut cur) = (l1, l2, 0, head.as_mut());
        while l1.is_some() || l2.is_some() {
            let mut num = carry;
            if let Some(node) = l1 {
                num += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                num += node.val;
                l2 = node.next;
            }
            carry = num / 10;
            cur.next = Some(Box::new(ListNode::new(num % 10)));
            cur = cur.next.as_mut().unwrap();
        }
        if carry > 0 { cur.next = Some(Box::new(ListNode::new(carry))); }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(Solution::add_two_numbers(linked![2,4,3], linked![5,6,4]), linked![7,0,8]);
        assert_eq!(Solution::add_two_numbers(linked![0], linked![0]), linked![0]);
        assert_eq!(Solution::add_two_numbers(linked![9,9,9,9,9,9,9], linked![9,9,9,9]), linked![8,9,9,9,0,0,0,1]);
    }
}