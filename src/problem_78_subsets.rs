struct Solution;


impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut subset: Vec<i32> = Vec::new();

        Self::dfs(&nums, &mut subset, &mut result, 0);

        result
        
    }

    fn dfs(
        nums: &Vec<i32>,
        subset: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        index: usize
    ) {

        if index >= nums.len() {
            result.push(subset.to_vec());
            return;
        }

        subset.push(nums[index]);

        Self::dfs(&nums, subset, result, index+1);

        subset.pop();

        Self::dfs(&nums, subset, result, index+1);

    }
}

mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3];

        assert_eq!(
            Solution::subsets(nums).sort(), 
            vec![
                Vec::new(), 
                vec![1], 
                vec![2],
                vec![1,2],
                vec![3],
                vec![1,3],
                vec![2,3],
                vec![1,2,3]
            ].sort()
        );
    }
}