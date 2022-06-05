use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
struct Solution {
    r: f64,
    xc: f64,
    yc: f64
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            r: radius,
            xc: x_center,
            yc: y_center
        }
    }
    
    fn rand_point(&self) -> Vec<f64> {
        let r = rand::thread_rng().gen::<f64>().sqrt();
        let theta = rand::thread_rng().gen::<f64>() * 2.0 * std::f64::consts::PI;
        let x = self.r * r * theta.sin() + self.xc;
        let y = self.r * r * theta.cos() + self.yc;
        vec![x, y]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
