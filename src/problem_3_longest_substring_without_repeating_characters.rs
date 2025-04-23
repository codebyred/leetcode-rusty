/*
Problem: Given a string s, find the length of the longest substring without repeating characters.

input choices:
s: string

representative values:
s = {
English letters: {"abc"}
Digits: {"123"}
Symbols: {"!?"}
Spaces: {"  "}
no characters: {""}
repeating characters: {"abcabcbb", "bbbbb", "pwwkew"}
non repeating characters: {"abc", "b"}
}

constraint: 0 <= s.lenth <= 5 * 10^4 (optimal time complexity for scanning input s is O(n))

test_cases:
Test case 1: English letters, repeating characters
input: "abcabcbb",
output: 3
expl: "abc" longest non repeating substr

Test case 2: English letters, repeating characters
input: "bbbbb",
output: 1
expl: "b" longest non repeating substr

Test case 3: English letters, repeating characters
input: "pwwkew"
output: 3
expl: "wke" longest non repeating substr

*/

use std::cmp::max;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        
        if s.len() == 0 { return 0 };
        if s.len() == 1 { return 1 };

        let mut item_idx_map: HashMap<char, i32> = HashMap::new();

        let mut before_idx = -1;

        let mut ans = 0;

        for (curr_idx, item) in s.chars().enumerate() {

            if let Some(last_idx) = item_idx_map.insert(item, curr_idx as i32) {

                before_idx = max(before_idx, last_idx);

            }

            ans = max(ans, before_idx - curr_idx as i32);

        }

        ans

        
    }
}

/*
sample:
input: "pwwkew"
"wke" is longest substr not "wkew" so we need a window. in this window no repeating charcters should exist
we can store the characters count in curr window 
if a character already exists in this window we reset the window size
, "pwke" is subsequence

algorithm:

we maintain a before so that we can find on which position the matching character existed
we maintain curr for our current index
we maintain ans that stores the substring length
we maintain a hashmap to remember encounter of a charcter


we scan thorugh the string by char
insert the char in map and get the last value

if a last value exist means there may be a duplicate. we first update before to the new last
now there is a problem. if we direcly update the before to new last this can cause issue as it would
store a index for before that is not latest. for example pwwkpew. now we encountered p, at this point the
window size is 3. but if we update before to previous encounter of p which is 0, then ans will be 4 which is
incorrect as there are two w's in the middle of this window. so before should calculated as max(before, last).
in this case last = 0, before = 1 which is where new window started.


ans is max of previous ans and windows size = (curr_character_pos - before_character_pos)
increment curr;



*/