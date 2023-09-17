use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        // Mapping our current int(n) to an current index(i)
        for (i, n) in nums.into_iter().enumerate() {
        // Subtracting our target and (n) 
            let diff = target - n;
        // Checking if the diff is in the HashMap and (j) holds the previous index
            if let Some(&j) = map.get(&diff) {
        // Returning index(i) and index(j) that add up to target
                return vec![i as i32, j as i32];
            } else {
        // Adding current number and index to Hashmap
                map.insert(n, i);
            }
        }
        unreachable!()
    }
}