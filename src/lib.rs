use std::ptr;

// #[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

const SIGNIFICANT_DIGITS: i32 = 2;
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let factor = 10.0f64.powi(SIGNIFICANT_DIGITS);
        f.debug_tuple("")
            .field(&((self.x * factor).trunc()/factor))
            .field( &((self.y * factor).trunc()/factor))
            .finish()
    }
}

pub fn is_convex_hull(points: &Vec<Point>, hull: &Vec<&Point>) -> bool {
    let filtered_list: Vec<&Point> = points.iter()
        .filter(|p| !hull.iter().any(|hull_p| ptr::eq(*p, *hull_p)))
        .collect();

    for i in 0..hull.len() {
        let p1 = hull[i];
        let p2 = hull[(i+1)%hull.len()];

        let line_dist_func = signed_distance_to_line(p1, p2);

        let all_below = filtered_list.iter()
            .map(|p| line_dist_func(*p) as f64)
            .all(|d:f64| d <= 0f64);

        if !all_below {
            return false;
        }
    }

    true
}
pub fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10.0f64.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}

pub fn signed_distance_to_line<'a>(a: &'a Point, b: &'a Point) -> Box<dyn Fn(&Point) -> f64 + 'a> {
    let mut plane_normal_x = a.y - b.y;
    let mut plane_normal_y = b.x - a.x;

    let norm = (plane_normal_x * plane_normal_x + plane_normal_y * plane_normal_y).sqrt();
    plane_normal_x = plane_normal_x / norm;
    plane_normal_y = plane_normal_y / norm;

    return Box::new(move |c: &Point| {
        let temp_x = c.x - a.x;
        let temp_y = c.y - a.y;
        temp_x * plane_normal_x + temp_y * plane_normal_y
    });

}
