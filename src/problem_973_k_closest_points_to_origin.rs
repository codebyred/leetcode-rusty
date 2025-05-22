use std::collections::BinaryHeap;
use std::cmp::Reverse;


struct Solution;

impl Solution {

    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        
        let mut v:Vec<(i32, usize)> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();

        for (index, row) in points.iter().enumerate() {
            let dis = row[0].pow(2) + row[1].pow(2);
            v.push((dis, index));
        };


        let mut min_heap = BinaryHeap::from(
            v.into_iter().map(Reverse).collect::<Vec<_>>()
        );

        let mut cur = k;

        while cur > 0 {
            let item = min_heap.pop().unwrap();

            let item1 = points[item.0.1][0];
            let item2 = points[item.0.1][1];

            ans.push(vec![item1, item2]);
            cur-=1;
        }

        ans

    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_1() {

        assert_eq!(Solution::k_closest(vec![
            vec![1,3], vec![-2,2]
        ], 1), vec![vec![-2,2]]);

    }
}

/*
    problems:
    1. store the euclidean distance
    2. sort based on euclidean distance
    3. create new vector and return
 */