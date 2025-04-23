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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut prev_node = None;
        let mut curr_node = head;

        while let Some(mut node) = curr_node {
            curr_node = node.next;
            node.next = prev_node;
            prev_node = Some(node);

        }

        prev_node

    }



}

// steps:
// two nodes: a and b, a->b; make a <- b; curr: a.next = b res: b.next = a
// - b.next will be lost, save b.next
// - make b.next = a, 
// problems: second nodes next may be none. 
// first node may be head node. if not head then it must point to its previous node

// sol:
// create a none node, make head point to it, take heads next and recursively call reverse.
// in fn will check if the node is null, then make a none node, make curr_node point to it, take it next and call reverse
// sol2
// create separate func, where we give info about prev node, so b node can point to a