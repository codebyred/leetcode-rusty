use std::collections::HashMap;

struct Solution;

impl Solution {

    fn two_sum(v: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map: HashMap<i32, i32> = HashMap::new();
    
    
        for (a_index, &a_val) in v.iter().enumerate() {
    
            let b_val = target - a_val;
    
            if let Some(&b_index) =  map.get(&b_val) {
    
                return vec![a_index as i32, b_index]
           
            }else{
    
                map.insert(a_val, a_index as i32);
    
            }
    
        }
    
        vec![]
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_problem_1_two_sum() {

        let test_vector = vec![2,7,11,15];
        let target = 9;
        let expected_result = vec![0,1];

        let mut result = Solution::two_sum(test_vector, target);
        result.sort();
        assert_eq!(expected_result, result);

        let test_vector = vec![3,2,4];
        let target = 6;
        let expected_result = vec![1,2];

        let mut result = Solution::two_sum(test_vector, target);
        result.sort();
        assert_eq!(expected_result, result);

    
        let test_vector = vec![3,3];
        let target = 6;
        let expected_result = vec![0, 1];

        let mut result = Solution::two_sum(test_vector, target);
        result.sort();
        assert_eq!(expected_result, result);

    }

}