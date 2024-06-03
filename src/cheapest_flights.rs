use std::collections::VecDeque;

pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Edge {
    pub weight: i32,
    pub vertex: i32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct Weight {
    pub weight: i32,
    pub distance: i32,
}

impl Default for Weight {
    fn default() -> Self {
        Self {
            weight: i32::MAX,
            distance: 0,
        }
    }
}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        flights.iter().for_each(|v| {
            let from = v[0];
            let to = v[1];
            let weight = v[2];
            graph[from as usize].push(Edge { vertex: to, weight });
        });
        let mut distance = vec![Weight::default(); n as usize];
        let mut in_queue = vec![false; n as usize];
        distance[src as usize].weight = 0;
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(src);
        in_queue[src as usize] = true;
        while let Some(node) = q.pop_front() {
            in_queue[node as usize] = false;
            let adj = &graph[node as usize];
            let from = distance[node as usize];
            for to in adj {
                if distance[to.vertex as usize].weight > from.weight + to.weight && from.distance <= k {
                    distance[to.vertex as usize] = Weight {
                        weight: from.weight + to.weight,
                        distance: from.distance + 1,
                    };

                    if !in_queue[to.vertex as usize] {
                        q.push_back(to.vertex);
                        in_queue[to.vertex as usize] = true;
                    }
                }
            }
        }
        if distance[dst as usize].weight < i32::MAX {
            distance[dst as usize].weight
        } else {
            -1
        }
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

    #[test]
    fn test4() {
        let n = 4;
        let flights = vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]];
        let src = 0;
        let dst = 3;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 6);
    }

    #[test]
    fn test5() {
        let n = 5;
        let flights = vec![
            vec![4, 1, 1],
            vec![1, 2, 3],
            vec![0, 3, 2],
            vec![0, 4, 10],
            vec![3, 1, 1],
            vec![1, 4, 3],
        ];
        let src = 2;
        let dst = 1;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), -1);
    }

    #[test]
    fn test6() {
        let n = 5;
        let flights = vec![
            vec![0, 1, 5],
            vec![1, 2, 5],
            vec![0, 3, 2],
            vec![3, 1, 2],
            vec![1, 4, 1],
            vec![4, 2, 1],
        ];
        let src = 0;
        let dst = 2;
        let k = 2;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 7);
    }

    #[test]
    fn test7() {
        let n = 3;
        let flights = vec![vec![0, 1, 2], vec![1, 2, 1], vec![2, 0, 10]];
        let src = 1;
        let dst = 2;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 1);
    }
}
