struct Solution;

fn merge_str(s1: &str, s2: &str) -> String {
    let mut s = String::from(s1);
    s.push_str(s2);
    s
}

fn gcd(mut m: usize, mut n: usize) -> usize {
    while n != 0 {
        let temp = m;
        m = n;
        n = temp % n;
    }
    m
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if merge_str(&str1, &str2) == merge_str(&str2, &str1) {
            str1[0..gcd(str1.len(), str2.len())].to_string()
        } else {
            String::new()
        }
    }
}
