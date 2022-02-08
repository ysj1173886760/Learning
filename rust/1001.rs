use std::collections::HashMap;
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mp: HashMap<(i32, i32), i32> = HashMap::new();
        let mut row: HashMap<i32, i32> = HashMap::new();
        let mut col: HashMap<i32, i32> = HashMap::new();
        let mut mainDiag: HashMap<i32, i32> = HashMap::new();
        let mut subDiag: HashMap<i32, i32> = HashMap::new();
        let mut ans = Vec::with_capacity(queries.len());

        for lamp in lamps {
            if (!mp.contains_key(&(lamp[0], lamp[1]))) {
                *mp.entry((lamp[0], lamp[1])).or_insert(0) += 1;
                *row.entry(lamp[0]).or_insert(0) += 1;
                *col.entry(lamp[1]).or_insert(0) += 1;
                *mainDiag.entry(lamp[0] + lamp[1]).or_insert(0) += 1;
                *subDiag.entry(lamp[0] - lamp[1]).or_insert(0) += 1;
            }
        }

        let dir = [(0, 1), (0, -1), (1, 1), (1, 0), (1, -1), (-1, 1), (-1, 0), (-1, -1), (0, 0)];
        for query in queries {
            // println!("{:?} {:?} {:?} {:?}", row, col, mainDiag, subDiag);
            if (row.contains_key(&query[0]) && *row.get(&query[0]).unwrap() > 0) ||
               (col.contains_key(&query[1]) && *col.get(&query[1]).unwrap() > 0) ||
               (mainDiag.contains_key(&(query[1] + query[0])) && *mainDiag.get(&(query[1] + query[0])).unwrap() > 0) ||
               (subDiag.contains_key(&(query[0] - query[1])) && *subDiag.get(&(query[0] - query[1])).unwrap() > 0) {
                ans.push(1i32);
            } else {
                ans.push(0i32);
            }
            for d in dir {
                let nx = query[0] + d.0;
                let ny = query[1] + d.1;
                if nx < 0 || nx >= n || ny < 0 || ny >= n {
                    continue;
                }
                if mp.contains_key(&(nx, ny)) {
                    *mp.get_mut(&(nx, ny)).unwrap() -= 1;
                    if *mp.get(&(nx, ny)).unwrap() == 0 {
                        mp.remove(&(nx, ny));
                    }
                    *row.get_mut(&nx).unwrap() -= 1;
                    *col.get_mut(&ny).unwrap() -= 1;
                    *mainDiag.get_mut(&(nx + ny)).unwrap() -= 1;
                    *subDiag.get_mut(&(nx - ny)).unwrap() -= 1;
                }
            }
        }
        ans
    }
}
