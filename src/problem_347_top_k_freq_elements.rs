use std::collections::HashMap;
struct  Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut ans = Vec::new();
        let mut map = HashMap::new();

        for item in &nums {
            map.entry(item)
                .and_modify(|value| {*value = *value + 1;})
                .or_insert(1);
        }

        let mut v: Vec<_> = map.into_iter().collect();
        v.sort_by_key(|&(_, v) | v);

        let mut cur = 0;

        for (&key, _) in v.iter().rev() {
            if cur >= k {break;}
            ans.push(key);
            cur = cur + 1;
        }

        ans
    }
}