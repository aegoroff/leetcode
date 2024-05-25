use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return 1;
        }
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        for num in &nums {
            let prev = num - 1;
            let next = num + 1;
            let has_prev = graph.contains_key(&prev);
            let has_next = graph.contains_key(&next);
            let adj = graph.entry(*num).or_insert(HashSet::new());
            if has_prev {
                adj.insert(prev);
            }
            if has_next {
                adj.insert(next);
            }
        }
        // Rust specific (cannot borrow mutable twice so do one more cycle)
        for num in &nums {
            let prev = num - 1;
            let next = num + 1;
            let has_prev = graph.contains_key(&prev);
            let has_next = graph.contains_key(&next);
            if has_prev {
                graph.entry(prev).and_modify(|p| {
                    p.insert(*num);
                });
            }
            if has_next {
                graph.entry(next).and_modify(|p| {
                    p.insert(*num);
                });
            }
        }
        let mut visited: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = Vec::with_capacity(nums.len());

        let mut count = 0;
        for n in &nums {
            if visited.contains(n) {
                continue;
            }
            stack.push(*n);
            while let Some(node) = stack.pop() {
                visited.insert(node);
                if let Some(adj) = graph.get(&node) {
                    for a in adj {
                        if visited.contains(a) {
                            continue;
                        }
                        count += 1;
                        stack.push(*a);
                    }
                }
            }
        }

        count + 1
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
