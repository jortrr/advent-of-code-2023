use std::{fmt::Debug, iter};

use colored::Colorize;

mod macros;

type Int = i32;
type Almanac = Vec<Vec<Transform>>;

#[derive(Clone)]
struct Interval {
    a: Int,
    b: Int,
    v: bool,
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interval")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

impl Interval {
    fn new(a: Int, b: Int) -> Interval {
        assert!(
            a <= b,
            "Invalid interval: a ({}) <= b ({}) must hold.",
            a,
            b
        );
        Interval { a, b, v: false }
    }

    fn single(a: Int) -> Interval {
        Interval { a, b: a, v: false }
    }

    fn len(&self) -> Int {
        self.b - self.a
    }

    fn split(&mut self, at: Int) -> Interval {
        assert!(at >= self.a && at <= self.b);
        let split_off_part = Interval::new(at, self.b);
        self.b = at - 1;
        split_off_part
    }

    fn shift(&mut self, by: Int) -> &mut Interval {
        self.a += by;
        self.b += by;
        self
    }

    fn apply(&self, t: &Transform) -> Vec<Interval> {
        let mut result: Vec<Interval> = vec![self.clone()];
        let mut transformed = false;

        if self.a >= t.source.a && self.b <= t.source.b {
            result[0].shift(t.get_shift());
            transformed = true;
        } else {
            let split_by_source_a = t.source.a > self.a && t.source.a <= self.b;
            let split_by_source_b = t.source.b < self.b && t.source.b >= self.a;

            if split_by_source_a {
                let mut first = result.pop().unwrap();
                let second = first.split(t.source.a);
                result = vec![first, second];
                transformed = true;
            }
            if split_by_source_b {
                let mut last = result.pop().unwrap();
                let second = last.split(t.source.b);
                result.push(last);
                result.push(second);
                transformed = true;
            }
            if split_by_source_a && split_by_source_b {
                result[1].shift(t.get_shift());
            } else if split_by_source_a {
                result[1].shift(t.get_shift());
            } else if split_by_source_b {
                result[0].shift(t.get_shift());
            }
        }
        if transformed {
            for i in &mut result {
                i.v = true;
            }
        }

        result
    }
}

#[derive(Debug)]
struct Transform {
    source: Interval,
    destination: Interval,
}

impl Transform {
    fn new(destination: Int, source: Int, length: Int) -> Transform {
        Transform {
            source: Interval::new(source, source + length),
            destination: Interval::new(destination, destination + length),
        }
    }

    fn get_shift(&self) -> Int {
        assert!(self.source.len() == self.destination.len());
        self.destination.a - self.source.a
    }

    fn apply(&self, intervals: Vec<Interval>) -> Vec<Interval> {
        let mut result: Vec<Interval> = Vec::new();
        for interval in intervals {
            if !interval.v {
                let new_intervals = interval.apply(self);
                result = result
                    .into_iter()
                    .chain(new_intervals.into_iter())
                    .collect();
            } else {
                result.push(interval);
            }
        }
        result
    }
}

fn apply_almanac(s: Vec<Interval>, a: &Almanac) -> Vec<Interval> {
    let mut s = s;
    for (i, transforms) in a.iter().enumerate() {
        for t in transforms {
            let old_s = s.clone();
            s = t.apply(s);
            if old_s != s {
                dbg!((old_s, t, &s));
            }
        }
        println!("[{}]: {:?}", i, &s);
        for i in &mut s {
            i.v = false;
        }
    }
    s
}

fn test(i: Interval, dest_a: Int, a: &Almanac) {
    let s = vec![i];
    let s = apply_almanac(s, a);
    test!(dest_a, s[0].a);
}

fn parse(e: Vec<String>) -> (Vec<Interval>, Almanac) {
    let mut s: Vec<Interval> = Vec::new();
    let mut a: Almanac = Vec::new();
    a.push(Vec::new());
    let mut c = 0;
    for l in e {
        if l.contains("seeds:") {
            let i: Vec<_> = l[6..]
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            for x in i {
                s.push(Interval::new(x, x));
            }
        } else if l.contains("map:") {
            a.push(Vec::new());
            if !a[c].is_empty() {
                c += 1;
            }
        } else if l.is_empty() {
            continue;
        } else {
            let i: Vec<_> = l
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            a[c].push(Transform::new(i[0], i[1], i[2]));
        }
    }
    (s, a)
}

fn main() {
    // Part 1 - Example
    let e = vec_of_strings![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];
    let (s, a) = parse(e);
    dbg!(&a);
    dbg!(&s);
    let s = apply_almanac(s, &a);
    dbg!(&s);
    test!(Interval::single(82), s[0]);
    test!(Interval::single(43), s[1]);
    test!(Interval::single(86), s[2]);
    test!(Interval::single(35), s[3]);
    //Part 1
}
