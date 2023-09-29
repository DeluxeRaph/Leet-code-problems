use std::collections::VecDeque; // Makes it easier to add things to the front of the vector
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: VecDeque<char> = VecDeque::new();
        let mut longest = 0;

        // C is the chars in the string
        for c in s.chars() {
            while set.contains(&c) {
                set.pop_front();
            }

            set.push_back(c);
            longest = longest.max(set.len());
        }

        longest as i32
    }
}