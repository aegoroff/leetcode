use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut visited: Vec<bool> = vec![false; n as usize];
        let mut graph: HashMap<i32, Vec<i32>> = (0..n).map(|n| (n, vec![])).collect();
        edges.iter().for_each(|v| {
            graph.get_mut(&v[0]).unwrap().push(v[1]);
            graph.get_mut(&v[1]).unwrap().push(v[0]);
        });

        Solution::dfs(&graph, &mut visited, source);

        visited[destination as usize]
    }

    fn dfs(graph: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>, v: i32) {
        if visited[v as usize] {
            return;
        }
        visited[v as usize] = true;
        if let Some(adj) = graph.get(&v) {
            for u in adj {
                Solution::dfs(graph, visited, *u);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::find_if_path_exists_in_grapsh::Solution;

    #[test]
    fn test1() {
        assert!(Solution::valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }
}
