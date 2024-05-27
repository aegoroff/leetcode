pub struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<i32>> = vec![vec![]; is_connected.len()];
        (0..is_connected.len()).for_each(|i| {
            for j in 0..is_connected[i].len() {
                if i == j || is_connected[i][j] == 0 {
                    continue;
                }
                graph[i].push(j as i32);
            }
        });
        let mut visited: Vec<i32> = vec![-1; is_connected.len()];
        let mut color = 0;
        for v in 0..is_connected.len() {
            if visited[v] == -1 {
                color += 1;
                dfs(&graph, &mut visited, v as i32, color);
            }
        }
        color
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
    use crate::num_of_provinces::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
