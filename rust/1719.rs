use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        for pair in pairs.iter() {
            adj.entry(pair[0]).or_insert(HashSet::new()).insert(pair[1]);
            adj.entry(pair[1]).or_insert(HashSet::new()).insert(pair[0]);
        }
        let mut root = -1;
        for (node, neighbours) in adj.iter() {
            if neighbours.len() == adj.len() - 1 {
                root = *node;
                break
            }
        }

        if root == -1 {
            return 0;
        }

        let mut res = 1;
        for (node, neighbours) in adj.iter() {
            if *node == root {
                continue;
            }
            let currDegree = neighbours.len();
            let mut parent = -1;
            let mut parentDegree = usize::MAX;

            for neighbour in neighbours {
                if adj.get(neighbour).unwrap().len() < parentDegree && adj.get(neighbour).unwrap().len() >= currDegree {
                    parent = *neighbour;
                    parentDegree = adj.get(neighbour).unwrap().len();
                }
            }
            if parent == -1 {
                return 0;
            }

            for neighbour in neighbours {
                if *neighbour == parent {
                    continue;
                }
                if !adj.get(&parent).unwrap().contains(neighbour) {
                    return 0;
                }
            }

            if parentDegree == currDegree {
                res = 2;
            }
        }
        res
    }
}
