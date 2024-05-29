use std::collections::VecDeque;

pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Edge {
    pub to: i32,
    pub weight: i32,
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
        let mut distance = vec![(0, 0); n as usize];
        let mut q = VecDeque::from([src]);
        while let Some(node) = q.pop_front() {
            let node = node as usize;
            let adj = &graph[node];
            let from_distance = distance[node];
            for a in adj {
                let to_distance = distance[a.to as usize];
                if to_distance.0 == 0 {
                    distance[a.to as usize] = (from_distance.0 + 1, from_distance.1 + a.weight);
                    q.push_back(a.to);
                }
            }
        }
        -1
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
