impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let len = points.len();
        // more generic
        let xr = &(0..len);
        let yr = &(0..len);
        let zr = &(0..len);
        xr.clone()
            .flat_map(move |x| yr.clone().flat_map(move |y| zr.clone().map(move |z| (x, y, z))))
            .map(|(x, y, z)| Self::get_area(&points[x], &points[y], &points[z]))
            .fold(0.0, |max, val| max.max(val))
    }
    fn get_area(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> f64 {
        (p1[0] * p2[1] + p2[0] * p3[1] + p3[0] * p1[1] - p1[0] * p3[1] - p2[0] * p1[1] - p3[0] * p2[1]).abs() as f64 * 0.5
    }
}
