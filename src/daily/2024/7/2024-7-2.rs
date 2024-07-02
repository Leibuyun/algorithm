struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        for n in nums1 {
            *m1.entry(n).or_insert(0) += 1;
        }
        for n in nums2 {
            *m2.entry(n).or_insert(0) += 1;
        }
        let mut res = vec![];
        for (k, v) in m1 {
            if let Some(v2) = m2.get(&k) {
                for _ in 0..v.min(*v2) {
                    res.push(k);
                }
            }
        }
        res
    }
}
