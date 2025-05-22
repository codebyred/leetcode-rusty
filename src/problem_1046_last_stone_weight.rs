use std::collections::BinaryHeap;


struct Solution;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {


        let mut heap = BinaryHeap::from(stones);

        
        while heap.len() >= 2 {

            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();

            if(x != y) {
                let new_weight = y - x;
                heap.push(new_weight);
            }

        }

        if heap.len() == 1 {
            return heap.pop().unwrap();
        }
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::last_stone_weight(vec![2,7,4,1,8,1]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }
}