use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let mut sums: Vec<i32> = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            if i == 0 {
                sums.push(nums[i]);
            } else if i == 1 {
                sums.push(cmp::max(nums[0], nums[i]));
            } else if sums[i - 2] + nums[i] < sums[i - 1] {
                sums.push(sums[i - 1]);
            } else {
                sums.push(sums[i - 2] + nums[i]);
            }
        }
        sums[sums.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::house_robbery::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
