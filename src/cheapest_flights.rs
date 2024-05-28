use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        flights.iter().for_each(|v| {
            let from = v[0];
            let to = v[1];
            let weight = v[2];
            graph[from as usize].push((to, weight));
        });
        let mut distance = vec![(0, -1); n as usize];
        let mut q = VecDeque::from([src]);
        while let Some(node) = q.pop_front() {
            let node = node as usize;
            let neighb = &graph[node];
            let to = distance[node];
            if to.0 == 0 {
                distance[node] = (to.0 + 1, neighb[0].1);
                q.push_back(neighb[0].0);
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
