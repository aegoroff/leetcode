pub struct Solution {}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        edges.iter().for_each(|v| {
            let from = v[0];
            let to = v[1];
            graph[from as usize].push(to);
            graph[to as usize].push(from);
        });

        let mut visited: Vec<i32> = vec![-1; n as usize];
        let mut color = 0;
        for v in 0..n as usize {
            if visited[v] == -1 {
                color += 1;
                dfs(&graph, &mut visited, v as i32, color);
            }
        }
        let mut components = vec![vec![]; color as usize];
        (0..n as usize).for_each(|i| {
            let c = visited[i];
            components[(c - 1) as usize].push(i);
        });

        let mut result = 0;
        for vertexes in &components {
            let n = vertexes.len();
            let n = n * (n - 1);
            let mut degree = 0;
            for v in vertexes {
                degree += graph[*v].len();
            }
            if degree == n {
                result += 1;
            }
        }

        result
    }
}

fn dfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<i32>, v: i32, color: i32) {
    visited[v as usize] = color;
    for u in &graph[v as usize] {
        let to_color = visited[*u as usize];
        if to_color == -1 {
            dfs(graph, visited, *u, color);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::count_complete_components::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]
            ),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
            ),
            1
        );
    }
}
