use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut visited: Vec<bool> = vec![false; n as usize];
        let mut graph: HashMap<i32, Vec<i32>> = (0..n).map(|n| (n, vec![])).collect();
        let mut all_visited_from: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(n as usize);
        edges.iter().for_each(|v| {
            graph.get_mut(&v[0]).unwrap().push(v[1]);
        });

        let mut sorted = Vec::with_capacity(n as usize);
        for v in 0..n {
            Solution::dfs(&graph, &mut visited, &mut all_visited_from, &mut sorted, v);
        }

        let mut result = vec![vec![]; n as usize];
        for (i, child) in sorted.iter().enumerate() {
            for ancestor in sorted[i + 1..sorted.len()].iter() {
                if let Some(path) = all_visited_from.get(ancestor) {
                    if path.contains(child) {
                        result[*child as usize].push(*ancestor);
                    }
                }
            }
            result[*child as usize].sort();
        }
        result
    }

    fn dfs(
        graph: &HashMap<i32, Vec<i32>>,
        visited: &mut Vec<bool>,
        all_visited_from: &mut HashMap<i32, HashSet<i32>>,
        sorted: &mut Vec<i32>,
        v: i32,
    ) {
        if visited[v as usize] {
            return;
        }
        all_visited_from.entry(v).or_default();

        visited[v as usize] = true;
        if let Some(adj) = graph.get(&v) {
            for u in adj {
                Solution::dfs(graph, visited, all_visited_from, sorted, *u);
                let childs = all_visited_from.get(u).unwrap().clone();
                if let Some(nodes) = all_visited_from.get_mut(&v) {
                    nodes.insert(*u);
                    nodes.extend(childs.iter());
                }
            }
        }
        sorted.push(v);
    }
}

#[cfg(test)]
mod test {
    use crate::all_ancestors::Solution;

    #[test]
    fn test1() {
        let result = Solution::get_ancestors(
            8,
            vec![
                vec![0, 3],
                vec![0, 4],
                vec![1, 3],
                vec![2, 4],
                vec![2, 7],
                vec![3, 5],
                vec![3, 6],
                vec![3, 7],
                vec![4, 6],
            ],
        );
        assert_eq!(
            result,
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3]
            ]
        );
    }

    #[test]
    fn test2() {
        let result = Solution::get_ancestors(
            5,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![0, 4],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
        );
        assert_eq!(
            result,
            vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]]
        );
    }

    #[test]
    fn test3() {
        let result = Solution::get_ancestors(
            52,
            vec![
                vec![30, 3],
                vec![30, 43],
                vec![30, 23],
                vec![30, 15],
                vec![30, 20],
                vec![30, 45],
                vec![30, 26],
                vec![30, 48],
                vec![30, 38],
                vec![30, 27],
                vec![30, 10],
                vec![30, 51],
                vec![30, 8],
                vec![30, 16],
                vec![30, 39],
                vec![30, 9],
                vec![30, 47],
                vec![30, 29],
                vec![30, 13],
                vec![30, 4],
                vec![30, 18],
                vec![30, 31],
                vec![30, 17],
                vec![30, 35],
                vec![30, 11],
                vec![30, 33],
                vec![30, 46],
                vec![30, 19],
                vec![30, 24],
                vec![30, 12],
                vec![30, 50],
                vec![30, 5],
                vec![30, 21],
                vec![30, 2],
                vec![30, 34],
                vec![30, 25],
                vec![30, 36],
                vec![3, 43],
                vec![3, 14],
                vec![3, 23],
                vec![3, 15],
                vec![3, 20],
                vec![3, 45],
                vec![3, 26],
                vec![3, 48],
                vec![3, 37],
                vec![3, 27],
                vec![3, 0],
                vec![3, 28],
                vec![3, 8],
                vec![3, 16],
                vec![3, 39],
                vec![3, 9],
                vec![3, 47],
                vec![3, 6],
                vec![3, 4],
                vec![3, 18],
                vec![3, 22],
                vec![3, 35],
                vec![3, 7],
                vec![3, 46],
                vec![3, 12],
                vec![3, 5],
                vec![3, 49],
                vec![3, 21],
                vec![3, 2],
                vec![3, 34],
                vec![3, 44],
                vec![43, 23],
                vec![43, 15],
                vec![43, 42],
                vec![43, 20],
                vec![43, 45],
                vec![43, 48],
                vec![43, 38],
                vec![43, 37],
                vec![43, 40],
                vec![43, 10],
                vec![43, 28],
                vec![43, 51],
                vec![43, 16],
                vec![43, 39],
                vec![43, 9],
                vec![43, 13],
                vec![43, 18],
                vec![43, 31],
                vec![43, 22],
                vec![43, 17],
                vec![43, 11],
                vec![43, 41],
                vec![43, 33],
                vec![43, 7],
                vec![43, 46],
                vec![43, 19],
                vec![43, 24],
                vec![43, 12],
                vec![43, 50],
                vec![43, 5],
                vec![43, 21],
                vec![43, 2],
                vec![43, 34],
                vec![43, 25],
                vec![43, 44],
                vec![14, 23],
                vec![14, 1],
                vec![14, 15],
                vec![14, 42],
                vec![14, 20],
                vec![14, 45],
                vec![14, 26],
                vec![14, 48],
                vec![14, 27],
                vec![14, 40],
                vec![14, 0],
                vec![14, 10],
                vec![14, 51],
                vec![14, 8],
                vec![14, 39],
                vec![14, 47],
                vec![14, 29],
                vec![14, 6],
                vec![14, 22],
                vec![14, 17],
                vec![14, 35],
                vec![14, 11],
                vec![14, 7],
                vec![14, 46],
                vec![14, 19],
                vec![14, 24],
                vec![14, 12],
                vec![14, 50],
                vec![14, 5],
                vec![14, 49],
                vec![14, 34],
                vec![14, 32],
                vec![14, 44],
                vec![23, 1],
                vec![23, 20],
                vec![23, 45],
                vec![23, 26],
                vec![23, 38],
                vec![23, 37],
                vec![23, 27],
                vec![23, 0],
                vec![23, 10],
                vec![23, 8],
                vec![23, 39],
                vec![23, 9],
                vec![23, 47],
                vec![23, 13],
                vec![23, 6],
                vec![23, 4],
                vec![23, 18],
                vec![23, 31],
                vec![23, 22],
                vec![23, 17],
                vec![23, 35],
                vec![23, 11],
                vec![23, 41],
                vec![23, 7],
                vec![23, 12],
                vec![23, 5],
                vec![23, 2],
                vec![23, 34],
                vec![23, 32],
                vec![23, 25],
                vec![23, 36],
                vec![23, 44],
                vec![1, 42],
                vec![1, 20],
                vec![1, 45],
                vec![1, 48],
                vec![1, 37],
                vec![1, 40],
                vec![1, 51],
                vec![1, 8],
                vec![1, 16],
                vec![1, 47],
                vec![1, 4],
                vec![1, 18],
                vec![1, 31],
                vec![1, 17],
                vec![1, 11],
                vec![1, 41],
                vec![1, 33],
                vec![1, 24],
                vec![1, 12],
                vec![1, 50],
                vec![1, 5],
                vec![1, 49],
                vec![1, 21],
                vec![1, 34],
                vec![1, 32],
                vec![1, 36],
                vec![1, 44],
                vec![15, 42],
                vec![15, 45],
                vec![15, 38],
                vec![15, 27],
                vec![15, 40],
                vec![15, 10],
                vec![15, 28],
                vec![15, 8],
                vec![15, 16],
                vec![15, 9],
                vec![15, 47],
                vec![15, 29],
                vec![15, 13],
                vec![15, 18],
                vec![15, 22],
                vec![15, 17],
                vec![15, 35],
                vec![15, 41],
                vec![15, 33],
                vec![15, 46],
                vec![15, 19],
                vec![15, 12],
                vec![15, 49],
                vec![15, 2],
                vec![15, 34],
                vec![15, 32],
                vec![15, 36],
                vec![42, 45],
                vec![42, 48],
                vec![42, 38],
                vec![42, 27],
                vec![42, 40],
                vec![42, 0],
                vec![42, 28],
                vec![42, 51],
                vec![42, 16],
                vec![42, 39],
                vec![42, 9],
                vec![42, 47],
                vec![42, 29],
                vec![42, 13],
                vec![42, 6],
                vec![42, 4],
                vec![42, 18],
                vec![42, 31],
                vec![42, 35],
                vec![42, 11],
                vec![42, 41],
                vec![42, 19],
                vec![42, 24],
                vec![42, 50],
                vec![42, 5],
                vec![42, 49],
                vec![42, 21],
                vec![42, 2],
                vec![42, 32],
                vec![42, 25],
                vec![20, 45],
                vec![20, 38],
                vec![20, 37],
                vec![20, 40],
                vec![20, 10],
                vec![20, 28],
                vec![20, 51],
                vec![20, 8],
                vec![20, 39],
                vec![20, 29],
                vec![20, 13],
                vec![20, 6],
                vec![20, 4],
                vec![20, 17],
                vec![20, 35],
                vec![20, 41],
                vec![20, 33],
                vec![20, 7],
                vec![20, 12],
                vec![20, 50],
                vec![20, 5],
                vec![20, 49],
                vec![20, 34],
                vec![20, 32],
                vec![20, 36],
                vec![20, 44],
                vec![45, 26],
                vec![45, 38],
                vec![45, 37],
                vec![45, 27],
                vec![45, 40],
                vec![45, 10],
                vec![45, 28],
                vec![45, 51],
                vec![45, 8],
                vec![45, 16],
                vec![45, 39],
                vec![45, 9],
                vec![45, 47],
                vec![45, 29],
                vec![45, 13],
                vec![45, 6],
                vec![45, 4],
                vec![45, 31],
                vec![45, 17],
                vec![45, 35],
                vec![45, 11],
                vec![45, 33],
                vec![45, 7],
                vec![45, 46],
                vec![45, 24],
                vec![45, 12],
                vec![45, 50],
                vec![45, 49],
                vec![45, 2],
                vec![45, 34],
                vec![45, 25],
                vec![45, 36],
                vec![45, 44],
                vec![26, 38],
                vec![26, 27],
                vec![26, 40],
                vec![26, 0],
                vec![26, 10],
                vec![26, 8],
                vec![26, 16],
                vec![26, 39],
                vec![26, 47],
                vec![26, 29],
                vec![26, 13],
                vec![26, 6],
                vec![26, 18],
                vec![26, 31],
                vec![26, 22],
                vec![26, 35],
                vec![26, 41],
                vec![26, 46],
                vec![26, 19],
                vec![26, 49],
                vec![26, 34],
                vec![26, 32],
                vec![26, 36],
                vec![26, 44],
                vec![48, 37],
                vec![48, 0],
                vec![48, 28],
                vec![48, 51],
                vec![48, 16],
                vec![48, 39],
                vec![48, 9],
                vec![48, 29],
                vec![48, 6],
                vec![48, 4],
                vec![48, 18],
                vec![48, 31],
                vec![48, 35],
                vec![48, 41],
                vec![48, 33],
                vec![48, 7],
                vec![48, 46],
                vec![48, 19],
                vec![48, 12],
                vec![48, 5],
                vec![48, 49],
                vec![48, 25],
                vec![48, 36],
                vec![48, 44],
                vec![38, 27],
                vec![38, 40],
                vec![38, 10],
                vec![38, 51],
                vec![38, 16],
                vec![38, 9],
                vec![38, 47],
                vec![38, 29],
                vec![38, 6],
                vec![38, 18],
                vec![38, 31],
                vec![38, 22],
                vec![38, 11],
                vec![38, 41],
                vec![38, 33],
                vec![38, 7],
                vec![38, 19],
                vec![38, 24],
                vec![38, 50],
                vec![38, 5],
                vec![38, 49],
                vec![38, 21],
                vec![38, 2],
                vec![38, 32],
                vec![38, 44],
                vec![37, 27],
                vec![37, 40],
                vec![37, 10],
                vec![37, 28],
                vec![37, 8],
                vec![37, 16],
                vec![37, 39],
                vec![37, 9],
                vec![37, 4],
                vec![37, 18],
                vec![37, 31],
                vec![37, 22],
                vec![37, 17],
                vec![37, 41],
                vec![37, 33],
                vec![37, 46],
                vec![37, 12],
                vec![37, 50],
                vec![37, 5],
                vec![37, 2],
                vec![37, 34],
                vec![37, 44],
                vec![27, 10],
                vec![27, 51],
                vec![27, 8],
                vec![27, 39],
                vec![27, 9],
                vec![27, 29],
                vec![27, 13],
                vec![27, 6],
                vec![27, 4],
                vec![27, 18],
                vec![27, 22],
                vec![27, 17],
                vec![27, 41],
                vec![27, 33],
                vec![27, 46],
                vec![27, 19],
                vec![27, 12],
                vec![27, 2],
                vec![27, 34],
                vec![27, 32],
                vec![27, 36],
                vec![27, 44],
                vec![40, 0],
                vec![40, 10],
                vec![40, 28],
                vec![40, 8],
                vec![40, 39],
                vec![40, 9],
                vec![40, 47],
                vec![40, 29],
                vec![40, 13],
                vec![40, 4],
                vec![40, 18],
                vec![40, 22],
                vec![40, 17],
                vec![40, 35],
                vec![40, 11],
                vec![40, 41],
                vec![40, 33],
                vec![40, 7],
                vec![40, 46],
                vec![40, 19],
                vec![40, 49],
                vec![40, 21],
                vec![40, 34],
                vec![40, 32],
                vec![40, 25],
                vec![40, 44],
                vec![0, 10],
                vec![0, 8],
                vec![0, 16],
                vec![0, 39],
                vec![0, 9],
                vec![0, 6],
                vec![0, 4],
                vec![0, 35],
                vec![0, 11],
                vec![0, 41],
                vec![0, 46],
                vec![0, 19],
                vec![0, 12],
                vec![0, 49],
                vec![0, 21],
                vec![0, 2],
                vec![0, 34],
                vec![0, 32],
                vec![0, 25],
                vec![0, 36],
                vec![0, 44],
                vec![10, 28],
                vec![10, 51],
                vec![10, 8],
                vec![10, 39],
                vec![10, 47],
                vec![10, 29],
                vec![10, 4],
                vec![10, 18],
                vec![10, 31],
                vec![10, 22],
                vec![10, 17],
                vec![10, 35],
                vec![10, 11],
                vec![10, 41],
                vec![10, 33],
                vec![10, 7],
                vec![10, 19],
                vec![10, 12],
                vec![10, 50],
                vec![10, 49],
                vec![10, 21],
                vec![10, 2],
                vec![10, 34],
                vec![10, 32],
                vec![10, 25],
                vec![10, 44],
                vec![28, 51],
                vec![28, 8],
                vec![28, 9],
                vec![28, 29],
                vec![28, 13],
                vec![28, 4],
                vec![28, 18],
                vec![28, 17],
                vec![28, 35],
                vec![28, 11],
                vec![28, 41],
                vec![28, 33],
                vec![28, 7],
                vec![28, 46],
                vec![28, 19],
                vec![28, 24],
                vec![28, 50],
                vec![28, 5],
                vec![28, 49],
                vec![28, 21],
                vec![28, 2],
                vec![28, 34],
                vec![28, 25],
                vec![28, 36],
                vec![51, 8],
                vec![51, 16],
                vec![51, 47],
                vec![51, 13],
                vec![51, 6],
                vec![51, 4],
                vec![51, 18],
                vec![51, 31],
                vec![51, 17],
                vec![51, 35],
                vec![51, 11],
                vec![51, 19],
                vec![51, 24],
                vec![51, 50],
                vec![51, 49],
                vec![51, 2],
                vec![51, 34],
                vec![51, 25],
                vec![51, 44],
                vec![8, 16],
                vec![8, 39],
                vec![8, 47],
                vec![8, 29],
                vec![8, 13],
                vec![8, 6],
                vec![8, 22],
                vec![8, 17],
                vec![8, 35],
                vec![8, 41],
                vec![8, 33],
                vec![8, 46],
                vec![8, 19],
                vec![8, 50],
                vec![8, 49],
                vec![8, 21],
                vec![8, 2],
                vec![8, 34],
                vec![8, 32],
                vec![8, 25],
                vec![8, 36],
                vec![8, 44],
                vec![16, 39],
                vec![16, 9],
                vec![16, 47],
                vec![16, 29],
                vec![16, 13],
                vec![16, 6],
                vec![16, 4],
                vec![16, 18],
                vec![16, 31],
                vec![16, 11],
                vec![16, 41],
                vec![16, 33],
                vec![16, 7],
                vec![16, 46],
                vec![16, 19],
                vec![16, 24],
                vec![16, 12],
                vec![16, 50],
                vec![16, 5],
                vec![16, 49],
                vec![16, 21],
                vec![16, 2],
                vec![16, 34],
                vec![16, 32],
                vec![16, 25],
                vec![16, 36],
                vec![39, 9],
                vec![39, 47],
                vec![39, 29],
                vec![39, 13],
                vec![39, 6],
                vec![39, 4],
                vec![39, 18],
                vec![39, 31],
                vec![39, 22],
                vec![39, 35],
                vec![39, 11],
                vec![39, 41],
                vec![39, 46],
                vec![39, 24],
                vec![39, 50],
                vec![39, 5],
                vec![39, 49],
                vec![39, 21],
                vec![39, 2],
                vec![39, 32],
                vec![39, 36],
                vec![39, 44],
                vec![9, 47],
                vec![9, 29],
                vec![9, 4],
                vec![9, 18],
                vec![9, 22],
                vec![9, 35],
                vec![9, 11],
                vec![9, 41],
                vec![9, 33],
                vec![9, 7],
                vec![9, 12],
                vec![9, 21],
                vec![9, 2],
                vec![9, 34],
                vec![9, 25],
                vec![9, 36],
                vec![9, 44],
                vec![47, 13],
                vec![47, 6],
                vec![47, 31],
                vec![47, 22],
                vec![47, 17],
                vec![47, 35],
                vec![47, 11],
                vec![47, 41],
                vec![47, 33],
                vec![47, 46],
                vec![47, 19],
                vec![47, 24],
                vec![47, 12],
                vec![47, 50],
                vec![47, 49],
                vec![47, 2],
                vec![47, 32],
                vec![47, 25],
                vec![47, 36],
                vec![29, 4],
                vec![29, 31],
                vec![29, 22],
                vec![29, 41],
                vec![29, 33],
                vec![29, 7],
                vec![29, 19],
                vec![29, 24],
                vec![29, 12],
                vec![29, 50],
                vec![29, 5],
                vec![29, 21],
                vec![29, 2],
                vec![29, 34],
                vec![29, 32],
                vec![29, 25],
                vec![29, 44],
                vec![13, 6],
                vec![13, 4],
                vec![13, 18],
                vec![13, 31],
                vec![13, 22],
                vec![13, 17],
                vec![13, 35],
                vec![13, 11],
                vec![13, 41],
                vec![13, 33],
                vec![13, 7],
                vec![13, 19],
                vec![13, 12],
                vec![13, 50],
                vec![13, 5],
                vec![13, 21],
                vec![13, 2],
                vec![13, 32],
                vec![13, 25],
                vec![13, 44],
                vec![6, 18],
                vec![6, 31],
                vec![6, 22],
                vec![6, 17],
                vec![6, 41],
                vec![6, 7],
                vec![6, 19],
                vec![6, 24],
                vec![6, 5],
                vec![6, 21],
                vec![6, 36],
                vec![6, 44],
                vec![4, 18],
                vec![4, 31],
                vec![4, 17],
                vec![4, 35],
                vec![4, 11],
                vec![4, 41],
                vec![4, 7],
                vec![4, 46],
                vec![4, 19],
                vec![4, 24],
                vec![4, 5],
                vec![4, 49],
                vec![4, 2],
                vec![4, 34],
                vec![4, 32],
                vec![4, 44],
                vec![18, 31],
                vec![18, 17],
                vec![18, 35],
                vec![18, 41],
                vec![18, 33],
                vec![18, 7],
                vec![18, 19],
                vec![18, 12],
                vec![18, 50],
                vec![18, 21],
                vec![18, 2],
                vec![18, 34],
                vec![18, 32],
                vec![18, 25],
                vec![18, 44],
                vec![31, 17],
                vec![31, 11],
                vec![31, 41],
                vec![31, 33],
                vec![31, 7],
                vec![31, 19],
                vec![31, 12],
                vec![31, 50],
                vec![31, 5],
                vec![31, 49],
                vec![31, 21],
                vec![31, 2],
                vec![31, 32],
                vec![31, 25],
                vec![31, 36],
                vec![22, 17],
                vec![22, 35],
                vec![22, 11],
                vec![22, 41],
                vec![22, 19],
                vec![22, 12],
                vec![22, 50],
                vec![22, 2],
                vec![22, 34],
                vec![22, 25],
                vec![22, 36],
                vec![22, 44],
                vec![17, 35],
                vec![17, 11],
                vec![17, 41],
                vec![17, 33],
                vec![17, 7],
                vec![17, 24],
                vec![17, 12],
                vec![17, 50],
                vec![17, 5],
                vec![17, 49],
                vec![17, 21],
                vec![17, 2],
                vec![17, 34],
                vec![17, 32],
                vec![17, 36],
                vec![17, 44],
                vec![35, 11],
                vec![35, 41],
                vec![35, 33],
                vec![35, 24],
                vec![35, 50],
                vec![35, 5],
                vec![35, 49],
                vec![35, 21],
                vec![35, 34],
                vec![35, 32],
                vec![35, 25],
                vec![35, 44],
                vec![11, 33],
                vec![11, 19],
                vec![11, 24],
                vec![11, 12],
                vec![11, 50],
                vec![11, 5],
                vec![11, 49],
                vec![11, 21],
                vec![11, 2],
                vec![11, 34],
                vec![11, 32],
                vec![11, 36],
                vec![11, 44],
                vec![41, 7],
                vec![41, 24],
                vec![41, 12],
                vec![41, 50],
                vec![41, 5],
                vec![41, 49],
                vec![41, 21],
                vec![41, 34],
                vec![41, 32],
                vec![41, 25],
                vec![41, 36],
                vec![41, 44],
                vec![33, 7],
                vec![33, 24],
                vec![33, 12],
                vec![33, 50],
                vec![33, 49],
                vec![33, 2],
                vec![33, 34],
                vec![33, 32],
                vec![33, 25],
                vec![33, 36],
                vec![33, 44],
                vec![7, 46],
                vec![7, 19],
                vec![7, 24],
                vec![7, 49],
                vec![7, 21],
                vec![7, 34],
                vec![7, 25],
                vec![46, 24],
                vec![46, 50],
                vec![46, 49],
                vec![46, 34],
                vec![46, 25],
                vec![46, 36],
                vec![19, 24],
                vec![19, 12],
                vec![19, 5],
                vec![19, 49],
                vec![19, 21],
                vec![19, 34],
                vec![19, 32],
                vec![19, 25],
                vec![19, 36],
                vec![19, 44],
                vec![24, 12],
                vec![24, 50],
                vec![24, 5],
                vec![24, 2],
                vec![24, 34],
                vec![24, 25],
                vec![24, 36],
                vec![24, 44],
                vec![12, 50],
                vec![12, 5],
                vec![12, 2],
                vec![12, 34],
                vec![12, 32],
                vec![12, 25],
                vec![12, 36],
                vec![50, 5],
                vec![50, 49],
                vec![50, 34],
                vec![50, 32],
                vec![50, 25],
                vec![50, 36],
                vec![50, 44],
                vec![5, 49],
                vec![5, 34],
                vec![5, 25],
                vec![5, 36],
                vec![5, 44],
                vec![49, 2],
                vec![49, 34],
                vec![49, 36],
                vec![49, 44],
                vec![21, 34],
                vec![21, 36],
                vec![21, 44],
                vec![2, 34],
                vec![2, 32],
                vec![2, 36],
                vec![2, 44],
                vec![34, 32],
                vec![34, 25],
                vec![34, 36],
                vec![32, 36],
                vec![36, 44],
            ],
        );
        assert_eq!(result.len(), 52);
    }
}
