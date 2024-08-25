struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut left = 0;
        let mut right = numbers.len() as i32 - 1;

        while left < right {

            let sum = numbers.get(left as usize).unwrap() + numbers.get(right as usize).unwrap();

            if sum == target {
                return vec![left+1, right+1]
            }
            else if sum < target {
                left += 1;
            }else {
                right -= 1;
            }
        }

        vec![]

    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_problem_167_two_sum() {
        assert_eq!(1,1);
    }
}