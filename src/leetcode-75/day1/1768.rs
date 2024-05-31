struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let words1 = word1.chars().collect::<Vec<char>>();
        let words2 = word2.chars().collect::<Vec<char>>();
        let mut res = String::new();
        let max = word1.len().max(word2.len());
        for i in 0..max {
            if i < words1.len() {
                res.push(words1[i]);
            }
            if i < words2.len() {
                res.push(words2[i]);
            }
        }
        res
    }
}
