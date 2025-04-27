use std::collections::VecDeque;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  #[allow(dead_code)]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {

        let mut tail = &mut head.as_mut().unwrap().next;
        let mut curr = tail.take();

        let mut deque: VecDeque<Option<Box<ListNode>>> = VecDeque::new();

        while curr.is_some() {

            let next = curr.as_mut().unwrap().next.take();

            deque.push_back(curr);

            curr = next;

        }

        let mut flag = false;

        while !deque.is_empty() {

            *tail = if flag {

                deque.pop_front().unwrap()

            }else {
               
                deque.pop_back().unwrap()
                
            };

            tail = &mut tail.as_mut().unwrap().next;

            flag = !flag;

        }
        
    }
}


mod tests{

    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn build_list(list: Vec<i32>) -> Option<Box<ListNode>> {

        let mut head: Option<Box<ListNode>> = None;

        for num in  list.into_iter().rev() {

            let mut node = Box::new(ListNode::new(num));

            node.next = head;

            head = Some(node);

        }
        

        head
    }
    #[allow(dead_code)]
    fn collect_list(mut head: &mut Option<Box<ListNode>>) -> Vec<i32> {

        let mut result = Vec::new();

        while head.is_some() {

            let curr = head.as_mut().unwrap();

            result.push(curr.val);

            head = &mut curr.next;

        }

        result

    }


    #[test]
    fn test_reorder_list() {
        let mut head = build_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut head);
        assert_eq!(collect_list(&mut head), vec![1, 4, 2, 3]);
    }

}