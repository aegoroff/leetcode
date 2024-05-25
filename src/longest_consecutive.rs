use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<_> = nums.into_iter().collect();

        nums.iter()
            .filter(|n| !nums.contains(&(*n - 1)))
            .map(|n| (*n..).take_while(|next| nums.contains(next)).count())
            .max()
            .unwrap_or_default() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::longest_consecutive::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_consecutive(vec![100,4,200,1,3,2]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_consecutive(vec![9,1,4,7,3,-1,0,5,8,-1,6]), 7);
    }
}
