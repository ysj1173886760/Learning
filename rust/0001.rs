use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate() {
            // because we may not have the key, so we need to use match to handle None situation
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, index);
                }
                Some(sub_index) => {
                    // we will get auto deference here, so it's ok whether you add * or not
                    // println!("index is {}", sub_index);
                    // println!("index is {}", *sub_index);
                    // but in vec!, we will not get auto deference, so you must add *
                    // i'm not sure about the exact reason yet...
                    return vec![*sub_index as i32, index as i32]
                }
            }
        }
        vec![]
    }
}