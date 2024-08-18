type Int = i64;

struct Point {
    x: Int,
    y: Int,
}

pub struct LineSegment {
    head: Point,
    tail: Point,
}

impl LineSegment {
    pub fn new(x1: Int, y1: Int, x2: Int, y2: Int) -> LineSegment {
        LineSegment {
            head: Point { x: x1, y: y1 },
            tail: Point { x: x2, y: y2 },
        }
    }
}

fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if val == 0 {
        return 0; // Collinear
    } else if val > 0 {
        return 1; // Clockwise
    } else {
        return 2; // Counterclockwise
    }
}

fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
    if q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y) {
        return true;
    }
    false
}

/// Returns true iff a and b intersect
pub fn intersects(a: &LineSegment, b: &LineSegment) -> bool {
    let p1 = &a.head;
    let q1 = &a.tail;
    let p2 = &b.head;
    let q2 = &b.tail;

    // Find the four orientations needed for the general and special cases
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special cases
    // p1, q1 and p2 are collinear and p2 lies on segment p1q1
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }

    // p1, q1 and q2 are collinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }

    // p2, q2 and p1 are collinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }

    // p2, q2 and q1 are collinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    // Doesn't fall in any of the above cases
    false
}
