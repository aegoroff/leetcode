use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut length = vec![0; nums.len()];
        for k in 0..nums.len() {
            length[k] = 1;
            for i in 0..k {
                if nums[i] < nums[k] {
                    length[k] = max(length[k], length[i] + 1);
                }
            }
        }
        length.sort();
        *length.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::longest_increasing_subseq::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::length_of_lis(vec![1, 2, 3]), 3);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::length_of_lis(vec![1, 2, -3, 4, 5]), 4);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::length_of_lis(vec![1, 2, -4, -3, -2, 4, 5]), 5);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::length_of_lis(vec![3, 2, 1]), 1);
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::length_of_lis(vec![1, 2, 3, 4, 0, 1, 5, 11]), 6);
    }

    #[test]
    fn test10() {
        assert_eq!(
            Solution::length_of_lis(vec![1, 2, 3, 4, 0, 1, 5, 11, 12, -3, -2, 15]),
            8
        );
    }

    #[test]
    fn test11() {
        assert_eq!(
            Solution::length_of_lis(vec![1, 4, 5, 2, 0, 1, 2, 11, 12, 3, 4, 5, 6, 5, 7, 8, 15]),
            10
        );
    }

    #[test]
    fn test12() {
        assert_eq!(Solution::length_of_lis(vec![9, 10, 0, 1, 2]), 3);
    }

    #[test]
    fn test13() {
        assert_eq!(
            Solution::length_of_lis(vec![3, 5, 6, 2, 5, 4, 19, 5, 6, 7, 12]),
            6
        );
    }
}
