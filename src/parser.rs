use crate::lib::Point;
use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;

pub(crate) fn parse_points(file: &String) -> io::Result<Vec<Point>> {

    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let mut line_iter = reader.lines();

    let amount: i64 = line_iter.next().unwrap()?.parse().unwrap();
    let mut points: Vec<Point> = Vec::with_capacity(amount as usize);
    for line in line_iter {
        if let Ok(line_str) = line {
            let numbers: Vec<f64> = line_str.split_whitespace().map(|c| c.parse().unwrap()).collect_vec();
            points.push(Point {
                x: numbers[1],
                y: numbers[2]
            })
        }
    }
    Ok(points)
}