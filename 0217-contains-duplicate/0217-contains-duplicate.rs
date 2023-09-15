use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // Creates the hash set
        let mut map = HashSet::new(); 
        // Iterates through nums first checking if an element in the vector matches an element in the hash set then returns true
        for &i in nums.iter(){
            if map.contains(&i){
                return true;
            }
        // If the element was not in the Hash set then the element is inserted into the set.
            map.insert(i);
        }
        false // Returns false if duplicates were not found
    }
}