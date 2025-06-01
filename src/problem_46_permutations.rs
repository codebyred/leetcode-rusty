struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let mut visited: Vec<bool> = vec![false; nums.len()];

        fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, visited: &mut Vec<bool>, ans: &mut Vec<Vec<i32>>) {
            if(path.len() == nums.len()) {
                ans.push(path.clone());
            }

            for (index,item) in nums.iter().enumerate() {
                if visited[index] {
                    continue;
                }

                visited[index] = true;

                path.push(nums[index]);

                backtrack(nums, path, visited, ans);

                path.pop();

                visited[index] = false;
            }
        }

        backtrack(&nums, &mut path, &mut visited, &mut ans);

        ans
    }
}

/*
problems:
    have to maintain order, each item can not come again
    so we have to somehow remember what was previously pushed to current state
    or move in a way that a item does not come again while building the permuation
solution:
    to remeber the path we can use hashmap or array of booleans
    at each call to backtrack we need to scan the nums array and find a number that is not yet came to
    our path. if we find that then we call backtrack again
*/
