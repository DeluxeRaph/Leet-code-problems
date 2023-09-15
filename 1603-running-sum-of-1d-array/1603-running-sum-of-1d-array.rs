
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums.clone();
    

        for i in 1..nums.len() {
            res[i] = nums[i] + res[i - 1];
        }

        res 
    }
}