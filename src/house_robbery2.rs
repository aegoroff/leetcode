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
                sums[i] = nums[i];
            } else if i == 1 {
                sums[i] = cmp::max(nums[1], nums[0]);
            } else if i == nums.len() - 1 {
                if nums.len() == 3 {
                    sums[i] = cmp::max(sums[i - 1], nums[i]);
                } else if sums[2] > sums[1] {
                    sums[i] = cmp::max(sums[i - 1], sums[i - 2] - nums[0] + nums[i]);
                } else {
                    sums[i] = cmp::max(sums[i - 2] + nums[i], sums[i - 1]);
                }
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
