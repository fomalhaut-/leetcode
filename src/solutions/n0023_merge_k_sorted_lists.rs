/**
 * [23] Merge k Sorted Lists
 *
 * You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
 * Merge all the linked-lists into one sorted linked-list and return it.
 *  
 * Example 1:
 * 
 * Input: lists = [[1,4,5],[1,3,4],[2,6]]
 * Output: [1,1,2,3,4,4,5,6]
 * Explanation: The linked-lists are:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 * 
 * Example 2:
 * 
 * Input: lists = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: lists = [[]]
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	k == lists.length
 * 	0 <= k <= 10^4
 * 	0 <= lists[i].length <= 500
 * 	-10^4 <= lists[i][j] <= 10^4
 * 	lists[i] is sorted in ascending order.
 * 	The sum of lists[i].length won't exceed 10^4.
 * 
 */
pub struct Solution {}
#[allow(unused_imports)]
use crate::models::linked_list::{ListNode, to_list};
use std::collections::BinaryHeap;

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut queue = BinaryHeap::new();
        for mut list in lists {
            while let Some(node) = list {
                list = node.next;
                queue.push(node.val);
            }
        }

        let mut head = Box::new(ListNode::new(0));
        let mut cur = head.as_mut();
        for val in queue.into_sorted_vec() {
            cur.next = Some(Box::new(ListNode::new(val)));
            cur = cur.next.as_mut().unwrap();
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(Solution::merge_k_lists(
            vec![
                to_list(vec![1,4,5]),
                to_list(vec![1,3,4]),
                to_list(vec![2,6]),
            ]),
           to_list(vec![1,1,2,3,4,4,5,6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]),
                   None
        );
    }
}