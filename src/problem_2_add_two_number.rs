// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution;
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = dummy_head.as_mut();
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {

            let val1 = l1.as_ref().map_or(0, |n| n.val);
            let val2 = l2.as_ref().map_or(0, |n| n.val);

            let sum = val1 + val2 + carry;

            carry = sum / 10;
            let digit = sum % 10;

            if let Some(t) = tail {
                t.next = Some(Box::new(ListNode::new(digit)));
                tail = t.next.as_mut();
            }

            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);

        }


        dummy_head.unwrap().next

    }
}

mod tests{
    use super::*;
    #[test]
    fn test_add_two_numbers(){

        let l1 = build_list(vec![2, 4, 3]);
        let l2 = build_list(vec![5, 6, 4]);
        assert_eq!(Solution::add_two_numbers(l1, l2), build_list(vec![7,0,8]));

    }

    #[test]
    fn test_add_two_numbers_xy(){

        let l1 = build_list(vec![9]);
        let l2 = build_list(vec![1,9,9,9,9,9,9,9,9,9]);
        assert_eq!(Solution::add_two_numbers(l1, l2), build_list(vec![0,0,0,0,0,0,0,0,0,0,1]));

    }
    fn build_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &num in nums.iter().rev() {
            let mut node = ListNode::new(num);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}