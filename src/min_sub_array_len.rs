pub struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prefixes: Vec<i32> = Vec::with_capacity(nums.len() + 1);

        for (i, x) in nums.iter().enumerate() {
            if i == 0 {
                prefixes.push(*x);
            } else {
                prefixes.push(prefixes[i - 1] + *x);
            }
        }
        prefixes.insert(0, 0);
        if let Some(v) = prefixes.last() {
            if *v < target {
                return 0;
            }
        }
        let mut r = 0;
        for l in 0..nums.len() {
            while r < nums.len() && prefixes[r + 1] - prefixes[l] < target {
                r += 1;
            }

            if r - l + 1 < result || result == 0 {
                result = r - l + 1;
            }

            if r == nums.len() {
                break;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use crate::min_sub_array_len::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::min_sub_array_len(7, vec![4, 3, 1, 2, 4, 4]), 2);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn test6() {
        let v = vec![10000; 100_000];
        assert_eq!(Solution::min_sub_array_len(1_000_000_000, v), 100_000);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::min_sub_array_len(3, vec![1, 1, 1]), 3);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::min_sub_array_len(5, vec![2, 3, 1, 1, 1, 1, 1]), 2);
    }
}
