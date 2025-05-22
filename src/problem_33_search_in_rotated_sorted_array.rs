struct Solution;
/*

}
*/

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
        let mut left: usize = 0;
        let mut right: usize = nums.len() -1 ;

        while left <= right {

            let mid = (left+right)/2;

            if target == nums[mid] {
                return mid as i32;
            }

            //check if left or ride side is sorted
            if nums[left] <= nums[mid] {

                if target >= nums[left] && target < nums[mid] {
                    right = mid - 1;
                }else {
                    left = mid + 1;
                }


            }else{

                if target > nums[mid] && target <= nums[right] {
                    left = mid + 1;
                }else {
                    right = mid - 1;
                }

            }

        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 0;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_example_4_found() {
        let nums = vec![6,7,0,1,2,3,4,5];
        let target = 3;
        assert_eq!(Solution::search(nums, target), 5);
    }

    #[test]
    fn test_example_5_no_rotation() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let target = 4;
        assert_eq!(Solution::search(nums, target), 3);
    }

    #[test]
    fn test_example_6_target_not_present() {
        let nums = vec![6,7,8,1,2,3,4];
        let target = 9;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
