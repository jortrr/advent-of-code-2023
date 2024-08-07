use std::{
    cmp::{max, min},
    iter::once,
};

use colored::{Colorize, CustomColor};
use grid::*;
use hex_color::HexColor;

mod grid;
mod macros;

type Int = i64;

#[derive(Debug)]
struct Point {
    point: grid::Point,
    terrain: Terrain,
}

impl Point {
    fn new(p: grid::Point, c: String) -> Point {
        Point {
            point: p,
            terrain: Terrain::Trench(c),
        }
    }
}

#[derive(Debug, Clone)]
enum Terrain {
    Ground,
    Trench(String),
}

impl Terrain {
    fn to_string(&self) -> String {
        use Terrain::*;
        match self {
            Ground => ".".to_string(),
            Trench(color) => {
                let c = HexColor::parse(&color).unwrap();
                "#".custom_color(CustomColor::new(c.r, c.g, c.b))
                    .to_string()
            }
        }
    }
}

#[derive(Debug)]
struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points: Vec::new() }
    }

    fn translate(self, x: Int, y: Int) -> Polygon {
        Polygon {
            points: self
                .points
                .iter()
                .map(|p| Point {
                    point: p.point.translate(x, y),
                    terrain: p.terrain.clone(),
                })
                .collect(),
        }
    }

    fn translate_to_px_py(self) -> Polygon {
        let mut min_x = Int::MAX;
        let mut min_y = Int::MAX;
        for p in &self.points {
            min_x = min(min_x, p.point.x);
            min_y = min(min_y, p.point.y);
        }
        self.translate(-min_x, -min_y)
    }

    fn print(&self) {
        let mut max_x = 0;
        let mut max_y = 0;
        for p in &self.points {
            max_x = max(max_x, p.point.x);
            max_y = max(max_y, p.point.y);
        }
        for y in 0..=max_y {
            for x in 0..=max_x {
                if let Some(p) = self
                    .points
                    .iter()
                    .find(|p| p.point.x == x && p.point.y == y)
                {
                    print!("{}", p.terrain.to_string());
                } else {
                    print!("{}", Terrain::Ground.to_string())
                }
            }
            println!();
        }
    }

    fn calc_area(&self) -> Int {
        //Shoelace formule to calculate area of polygon
        if self.points.is_empty() {
            0
        } else {
            let points: Vec<_> = self.points.iter().chain(once(&self.points[0])).collect();
            let b = {
                let mut sum = 0;
                for i in 0..points.len() - 1 {
                    let (p_0, p_1) = (points[i].point, points[i + 1].point);
                    sum += p_0.distance_to(&p_1) as Int;
                }
                sum
            };
            let mut area: Int = 0;
            for i in 0..self.points.len() {
                let (p_0, p_1) = (points[i].point, points[i + 1].point);
                area += p_0.x * p_1.y - p_1.x * p_0.y;
            }
            area = (area / 2).abs();
            // A = i + b/2 - 1
            // i = A-b/2+1
            // Filled polygon = i + b
            b + area - b / 2 + 1
        }
    }

    fn from_dig_plan(dig_plan: &Vec<String>, swapped: bool) -> Polygon {
        let mut polygon: Polygon = Polygon::new();
        let mut p = grid::Point::new(0, 0);
        for op in dig_plan {
            let (mut d, mut l, c) = {
                let mut i = op.split_whitespace();
                (
                    i.next().unwrap(),
                    i.next().unwrap().parse::<Int>().unwrap(),
                    i.next().unwrap(),
                )
            };
            let c = &c[1..c.len() - 1].to_string();
            if swapped {
                let hex_l = &c[1..6];
                let hex_d = &c[6..7];
                l = Int::from_str_radix(hex_l, 16).unwrap();
                d = match hex_d {
                    "0" => "R",
                    "1" => "D",
                    "2" => "L",
                    "3" => "U",
                    _ => panic!("Invalid hex direction: '{}'", hex_d),
                };
            }
            let direction = match d {
                "R" => East,
                "D" => South,
                "U" => North,
                "L" => West,
                _ => panic!("Invalid direction: '{}'", d),
            };
            if swapped {
                polygon.points.push(Point::new(p, c.clone()));
                p = p.move_distance(&direction, l);
            } else {
                for _ in 0..l {
                    polygon.points.push(Point::new(p, c.clone()));
                    p = p.move_to(&direction);
                }
            }
        }
        debug!(false, "{:?}", &polygon);
        polygon = polygon.translate_to_px_py();
        polygon
    }
}

fn main() {
    println!("Hello, World! from src/day18.rs!");
    // Part 1 - Example
    let dig_plan = vec_of_strings![
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ];
    let polygon = Polygon::from_dig_plan(&dig_plan, false);
    dbg!(&polygon);
    polygon.print();
    let a = polygon.calc_area();
    test!(62, a);
    // Part 1
    let i = aoc::get(2023, 18);
    let polygon = Polygon::from_dig_plan(&i, false);
    let a = polygon.calc_area();
    test!(48652, a);
    // Part 2 - Example
    let polygon = Polygon::from_dig_plan(&dig_plan, true);
    //dbg!(&polygon);
    let a = polygon.calc_area();
    test!(952408144115 as i64, a);
    // Part 2
    let polygon = Polygon::from_dig_plan(&i, true);
    let a = polygon.calc_area();
    test!(45757884535661 as i64, a);
}
