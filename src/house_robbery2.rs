use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 2 {
            return cmp::max(nums[1], nums[0]);
        }
        let mut sums: Vec<i32> = vec![0; nums.len()];
        for i in 0..nums.len() {
            if i == 0 {
                let pre_last = nums.len() - 2 % nums.len();
                sums[i] = cmp::max(nums[pre_last] + nums[i], nums[nums.len() - 1]);
            } else if i == 1 {
                sums[i] = cmp::max(nums[nums.len() - 1] + nums[i], sums[0]);
            } else {
                sums[i] = cmp::max(sums[i - 2] + nums[i], sums[i - 1]);
            }
        }
        sums[sums.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::house_robbery2::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::rob(vec![1, 2, 1, 1]), 3);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::rob(vec![1, 2, 2, 1, 1]), 3);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::rob(vec![1, 2]), 2);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::rob(vec![200, 3, 140, 20, 10]), 340);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::rob(vec![1, 1, 1, 2]), 3);
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 3);
    }
}
