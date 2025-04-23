use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        
        if s.len() != t.len() {
            return false;
        }
        
        let mut letter_count_map: HashMap<char, i32> = HashMap::new();
    
        for letter in s.chars() {
            letter_count_map
                .entry(letter)
                .and_modify(|val| *val += 1)
                .or_insert(1);
    
        }
        
        for letter in t.chars() {
            letter_count_map
                .entry(letter)
                .and_modify(|val| *val -= 1 );
       
        }
        
        for (_, val) in letter_count_map.iter() {
            if *val > 0 {
                return false;
            }
        }
        
        true
    }
}