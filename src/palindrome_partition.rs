pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = vec![];
        let single_chars = s.chars().map(|c| c.to_string()).collect();
        result.push(single_chars);
        for len in 1..s.len() {
            let mut p = vec![];
            if Solution::is_palindrome(&s[0..len]) && Solution::is_palindrome(&s[len..s.len()]) {
                p.push(s[0..len].to_string());
                p.push(s[len..s.len()].to_string());
            }
            if p.iter().any(|s| s.len() > 1) {
                result.push(p);
            }
        }
        if s.len() > 1 && Solution::is_palindrome(&s) {
            result.push(vec![s]);
        }

        result
    }

    pub fn is_palindrome(s: &str) -> bool {
        let forward = s.chars();
        let mut backward = s.chars().rev();
        for f in forward {
            if let Some(b) = backward.next() {
                if b != f {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::palindrome_partition::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::partition("aab".to_string()),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::partition("a".to_string()), vec![vec!["a"],]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::partition("ab".to_string()), vec![vec!["a", "b"],]);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::partition("bb".to_string()),
            vec![vec!["b", "b"], vec!["bb"]]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::partition("abbab".to_string()),
            vec![
                vec!["a", "b", "b", "a", "b"],
                vec!["a", "b", "bab"],
                vec!["a", "bb", "a", "b"],
                vec!["abba", "b"]
            ]
        );
    }

    #[test]
    fn is_palindrome_false() {
        assert!(!Solution::is_palindrome("aab"));
    }

    #[test]
    fn is_palindrome_true() {
        assert!(Solution::is_palindrome("aabaa"));
    }

    #[test]
    fn is_palindrome_single_true() {
        assert!(Solution::is_palindrome("a"));
    }
}
