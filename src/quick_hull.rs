
use std::ptr;
use itertools::Either;
use crate::lib::{ Point, approx_equal, signed_distance_to_line};
use rayon::prelude::*;

pub fn get_convex_hull(points: &Vec<Point>) -> Vec<&Point> {
    if points.len() <= 2 {
        return points.iter().map(|p| p).collect();
    }

    let mut max_x: &Point = &points[0];
    let mut min_x: &Point = &points[0];

    for point in points {
        if max_x.x < point.x || (approx_equal(max_x.x, point.x, 6) && max_x.y < point.y) {
            max_x = point;
        }
        if min_x.x > point.x || (approx_equal(min_x.x, point.x, 6) && min_x.y > point.y) {
            min_x = point;
        }
    }

    let line_dist_func = signed_distance_to_line(min_x, max_x);

    let (above, below): (Vec<(&Point, f64)>, Vec<(&Point, f64)>) = points.par_iter()
        .filter(|p| !ptr::eq(*p, max_x) && !ptr::eq(*p, min_x))
        .map(|p| (p, line_dist_func(p) as f64))
        .partition(|(_, d)| *d > 0f64);

    let below_positive = below.par_iter().map(|(p, d)|(*p, -d)).collect();
    let below_hull = get_hull(below_positive, max_x, min_x);
    let above_hull = get_hull(above, min_x, max_x);
    let mut hull = vec![min_x];
    hull.extend(above_hull);
    hull.push(max_x);
    hull.extend(below_hull);

    return hull;
}

fn get_hull<'a>(relevant_points:Vec<(&'a Point, f64)>, point_a: &'a Point, point_b: &'a Point) -> Vec<&'a Point> {
    if relevant_points.is_empty() {
        return vec![];
    }

    let (highest, _): &(&Point, _) = relevant_points.par_iter()
        .reduce(|| &relevant_points[0], |p1, p2| if p1.1 > p2.1 {p1} else {p2});

    //          highest
    //            / \
    //           /   \
    //          /     \
    //       0 /       \ 1
    // LEFT   / INSIDE  \    RIGHT
    //       A --------- B

    let line_0_dist = signed_distance_to_line(point_a, highest);
    let line_1_dist = signed_distance_to_line(highest, point_b);

    let (left, right): (Vec<(&Point, f64)>, Vec<(&Point, f64)>) = relevant_points.par_iter()
        .map(|(p, _)| *p)
        .filter(|p| !ptr::eq(*p, *highest))
        .filter_map(|p| to_location(p, line_0_dist(p), line_1_dist(p)))
        .partition_map(|p| p);

    let mut left_points = get_hull(left, point_a, highest);
    let right_points = get_hull(right,  highest, point_b);

    left_points.push(highest);
    left_points.extend(right_points);
    return left_points;
}

fn to_location(point: &Point, dist_0: f64, dist_1: f64) -> Option<Either<(&Point, f64), (&Point, f64)>> {
    match (dist_0 < 0f64, dist_1 < 0f64) {
        (false, true) => Some(Either::Left((point, dist_0))),
        (true, false) => Some(Either::Right((point, dist_1))),
        (_, _) => None
    }
}

// enum Location<'a> {
//     Left(&'a Point, f64),
//     Right(&'a Point, f64),
// }


#[cfg(test)]
mod tests {
    use crate::lib::Point;
    use crate::quick_hull::get_convex_hull;
    use crate::lib::is_convex_hull;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use std::time::Instant;

    const SEED: u64 = 0xdeadbeef;

    fn create_points(seed: u64, amount: i64, width: f64, height: f64) -> Vec<Point> {
        let mut rng = StdRng::seed_from_u64(seed);

        return (0..amount).map(|_| Point {
            x: rng.gen::<f64>() * width - width / 2f64,
            y: rng.gen::<f64>() * height - height / 2f64
        }).collect();

    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn simple_test() {
        let points = vec![
            Point { x: 0f64, y: 0f64 },
            Point { x: 0f64, y: 1f64 },
            Point { x: 1f64, y: 0f64 },
            Point { x: 1f64, y: 1f64 },
        ];

        let hull = get_convex_hull(&points);
        assert!(is_convex_hull(&points, &hull));

    }

    #[test]
    fn test_5() {
        for i in 0..50  {
            let points = create_points(SEED + i, 5, 100f64, 100f64);
            let hull = get_convex_hull(&points);
            println!("Points: {:?}", points);
            println!("Hull: {:?}", hull);
            assert!(is_convex_hull(&points, &hull), "Problem in iteration {}", i);
        }
    }

    #[test]
    fn test_10() {
        for i in 0..50  {
            let points = create_points(SEED + i, 10, 100f64, 100f64);
            let hull = get_convex_hull(&points);
            println!("Points: {:?}", points);
            println!("Hull: {:?}", hull);
            assert!(is_convex_hull(&points, &hull), "Problem in iteration {}", i);
        }
    }

    #[test]
    fn test_20() {
        for i in 0..50  {
            let points = create_points(SEED + i, 20, 100f64, 100f64);
            let hull = get_convex_hull(&points);
            println!("Points: {:?}", points);
            println!("Hull: {:?}", hull);
            assert!(is_convex_hull(&points, &hull), "Problem in iteration {}", i);
        }
    }

    #[test]
    fn test_50() {
        for i in 0..50  {
            let points = create_points(SEED + i, 50, 100f64, 100f64);
            let hull = get_convex_hull(&points);
            println!("Points: {:?}", points);
            println!("Hull: {:?}", hull);
            assert!(is_convex_hull(&points, &hull), "Problem in iteration {}", i);
        }
    }

    #[test]
    fn test_100() {
        for i in 0..50  {
            let points = create_points(SEED + i, 100, 100f64, 100f64);
            let hull = get_convex_hull(&points);
            println!("Points: {:?}", points);
            println!("Hull: {:?}", hull);
            assert!(is_convex_hull(&points, &hull), "Problem in iteration {}", i);
        }
    }

    #[test]
    fn test_5_no_2() {
        let i = 2;
        let points = create_points(SEED + i, 5, 100f64, 100f64);
        let hull = get_convex_hull(&points);
        println!("Points: {:?}", points);
        println!("Hull: {:?}", hull);
        assert!(is_convex_hull(&points, &hull), "Problem in iteration {}", i);
    }

    #[test]
    fn big_test() {
        let points = create_points(SEED, 10_000_000, 100_f64, 100_f64);
        let now = Instant::now();
        let hull = get_convex_hull(&points);
        println!("Elapsed time: {} ms", now.elapsed().as_millis());
        // without parallelization Elapsed time: 2959 ms
        // with parallelization: Elapsed time: 1245 ms
        assert!(is_convex_hull(&points, &hull), "Could not find correct hull for big test");
    }
}