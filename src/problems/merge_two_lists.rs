use crate::problems::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    /// You are given the heads of two sorted linked lists `list1` and `list2`.
    ///
    /// Merge the two lists into one *sorted* list. The list should be made by
    /// splicing together the nodes of the first two lists.
    ///
    /// Return the head of the merged linked list.
    #[allow(unused)]
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Solution::recursively_merge_lists(list1.as_ref(), list2.as_ref());
    }

    fn recursively_merge_lists(
        list1: Option<&Box<ListNode>>,
        list2: Option<&Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_node = None;
        match (list1, list2) {
            (None, None) => {}
            (Some(a), None) => {
                new_node = Some(Box::new(ListNode::new(a.val)));
                new_node.as_mut().unwrap().next =
                    Solution::recursively_merge_lists(a.next.as_ref(), None);
            }
            (None, Some(b)) => {
                new_node = Some(Box::new(ListNode::new(b.val)));
                new_node.as_mut().unwrap().next =
                    Solution::recursively_merge_lists(None, b.next.as_ref());
            }
            (Some(a), Some(b)) => {
                if b.val > a.val {
                    new_node = Some(Box::new(ListNode::new(a.val)));
                    new_node.as_mut().unwrap().next =
                        Solution::recursively_merge_lists(a.next.as_ref(), list2);
                } else {
                    new_node = Some(Box::new(ListNode::new(b.val)));
                    new_node.as_mut().unwrap().next =
                        Solution::recursively_merge_lists(list1, b.next.as_ref());
                }
            }
        }
        return new_node;
    }
}

#[cfg(test)]
fn list_node_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    if vec.len() == 0 {
        return None;
    }
    let mut new_list_node_root = ListNode::new(vec[0]);
    let mut i = 1;
    let mut current_list_node = &mut new_list_node_root;
    while i < vec.len() {
        let next_node = ListNode {
            val: vec[i],
            next: None,
        };
        current_list_node.next = Some(Box::new(next_node));
        current_list_node = current_list_node.next.as_mut().unwrap().as_mut();
        i += 1;
    }

    return Some(Box::new(new_list_node_root));
}

#[cfg(test)]
fn test_output(raw_list1: Vec<i32>, raw_list2: Vec<i32>, raw_output: Vec<i32>) {
    let list1 = list_node_from_vec(raw_list1);
    let list2 = list_node_from_vec(raw_list2);
    let output = list_node_from_vec(raw_output);

    assert_eq!(Solution::merge_two_lists(list1, list2), output);
}

#[test]
fn test_01() {
    test_output(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]);
}

#[test]
fn test_02() {
    test_output(vec![], vec![], vec![]);
}

#[test]
fn test_03() {
    test_output(vec![], vec![0], vec![0]);
}
