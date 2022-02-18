impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let minn0 = std::cmp::min(edges[0][0], edges[0][1]);
        let minn1 = std::cmp::min(edges[1][0], edges[1][1]);
        let maxx0 = std::cmp::max(edges[0][0], edges[0][1]);
        let maxx1 = std::cmp::max(edges[1][0], edges[1][1]);
        if minn0 == minn1 {
            minn0
        } else {
            maxx0
        }
    }
}
