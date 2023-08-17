impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut scores: Vec<i32> = Vec::new();

        for op in operations {
            match op.as_str() {
                "+" => {
                    let last = *scores.last().unwrap();
                    let second_last = *scores.get(scores.len() - 2).unwrap();
                    scores.push(last + second_last);
                },
                "D" => {
                    let last = *scores.last().unwrap();
                    scores.push(2 * last);
                },
                "C" => {
                    scores.pop();
                },
                _ => {
                    scores.push(op.parse::<i32>().unwrap());
                },
            }
        }

        scores.iter().sum()
    }
}