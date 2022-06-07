impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let v1 = (points[0][0] - points[1][0], points[0][1] - points[1][1]);
        let v2 = (points[0][0] - points[2][0], points[0][1] - points[2][1]);
        v1.0 * v2.1 - v1.1 * v2.0 != 0
    }
}
