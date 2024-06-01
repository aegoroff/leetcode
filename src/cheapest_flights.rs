use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Edge {
    pub weight: i32,
    pub to: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.to.cmp(&other.to))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        flights.iter().for_each(|v| {
            let from = v[0];
            let to = v[1];
            let weight = v[2];
            graph[from as usize].push(Edge { to, weight });
        });
        let mut distance = vec![(i32::MAX, 0); n as usize];
        let mut q = BinaryHeap::from([Edge { to: src, weight: 0 }]);
        while let Some(node) = q.pop() {
            let adj = &graph[node.to as usize];
            if node.weight > distance[node.to as usize].0 {
                continue;
            }
            for a in adj {
                let next = Edge {
                    to: a.to,
                    weight: node.weight + a.weight,
                };

                let to_distance = distance[a.to as usize];
                if next.weight < to_distance.0 && to_distance.1 <= k {
                    distance[a.to as usize] = (next.weight, to_distance.1 + 1);
                    q.push(next);
                }
            }
        }
        distance[dst as usize].0
    }
}

#[cfg(test)]
mod test {
    use crate::cheapest_flights::Solution;

    #[test]
    fn test1() {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 700);
    }

    #[test]
    fn test2() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 200);
    }

    #[test]
    fn test3() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 500);
    }
}
