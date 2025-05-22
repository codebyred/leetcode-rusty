use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct KthLargest {
    k: i32,
    heap: BinaryHeap<Reverse<i32>>
}

/*

*/

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        
        let mut heap: BinaryHeap<Reverse<i32>> = nums.into_iter().map(Reverse).collect();

        while(heap.len() as i32> k) {
            heap.pop();
        }

        Self {
            k,
            heap
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        
        self.heap.push(Reverse(val));

        if self.heap.len() as i32 > self.k {
            self.heap.pop();
        }

        if let Some(val) = self.heap.peek() {
            val.0
        }else {
            0
        }

    }
}
