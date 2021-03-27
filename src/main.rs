mod quick_hull;
mod lib;
mod parser;

use crate::quick_hull::get_convex_hull;
use std::env;
use crate::parser::parse_points;
use std::time::Instant;
use serde_json::{json, to_string_pretty, Value};

fn main() -> Result<(), std::io::Error> {

    let args: Vec<String> = env::args().collect();
    let points = parse_points(&args[1])?;
    let instant = Instant::now();
    let hull = get_convex_hull(&points);
    // println!("Hull: {:?}", hull);
    let time_millis = instant.elapsed().as_millis();

    let output: Value = json!({
        "hullSize": hull.len(),
        "convexHull": hull,
        "timing": time_millis as u64
    });
    println!("{}", to_string_pretty(&output)?);
    Ok(())
}
