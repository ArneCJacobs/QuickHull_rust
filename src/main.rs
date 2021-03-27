mod quick_hull;
mod lib;
use crate::quick_hull::get_convex_hull;
use crate::lib::Point;

fn main() {

    let points = vec![
        Point { x: 0f64, y: 0f64 },
        Point { x: 0f64, y: 1f64 },
        Point { x: 1f64, y: 0f64 },
        Point { x: 1f64, y: 1f64 },
    ];
    println!("Hull: {:?}", get_convex_hull(&points));
}
