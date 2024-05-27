pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut low = 0;
        let mut high = 46340;
        let mut size = high - low;
        while size > 1 {
            let mid = low + size / 2;
            if mid * mid > num {
                high = mid;
            } else {
                low = mid;
            }
            size = high - low;
        }
        low * low == num || high * high == num
    }
}

#[cfg(test)]
mod test {
    use crate::valid_perfect_square::Solution;

    #[test]
    fn test1() {
        assert!(Solution::is_perfect_square(16));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_perfect_square(14));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_perfect_square(2147395600));
    }
}
