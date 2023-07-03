use core::cmp::{max, min};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

#[derive(Debug, Clone)]
struct Rect {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Rect {
    fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Rect {
        assert!(x1 <= x2);
        assert!(y1 <= y2);
        return Rect {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        };
    }

    fn intersect(&self, other: &Rect) -> Option<Rect> {
        let x1 = max(self.x1, other.x1);
        let y1 = max(self.y1, other.y1);
        let x2 = min(self.x2, other.x2);
        let y2 = min(self.y2, other.y2);
        if x1 <= x2 && y1 <= y2 {
            return Some(Rect::new(x1, y1, x2, y2));
        } else {
            return None;
        }
    }

    fn contains(&self, point: &Point) -> bool {
        return self.x1 <= point.x
            && self.x2 >= point.x
            && self.y1 <= point.y
            && self.y2 >= point.y;
    }

    fn bounding_rect(points: &Vec<Point>) -> Rect {
        let mut x1 = points[0].x;
        let mut y1 = points[0].y;
        let mut x2 = points[0].x;
        let mut y2 = points[0].y;
        for p in points {
            x1 = min(x1, p.x);
            y1 = min(y1, p.y);
            x2 = max(x2, p.x);
            y2 = max(y2, p.y);
        }
        return Rect::new(x1, y1, x2, y2);
    }

    fn add(&self, d: i64) -> Rect {
        return Rect::new(self.x1 - d, self.y1 - d, self.x2 + d, self.y2 + d);
    }
}

#[derive(Debug, Clone)]
enum QuadTree {
    Node {
        rect: Rect,
        t1: Box<QuadTree>,
        t2: Box<QuadTree>,
        t3: Box<QuadTree>,
        t4: Box<QuadTree>,
    },
    Leaf {
        rect: Rect,
        points: Vec<Point>,
    },
}

impl QuadTree {
    fn new_node(rect: Rect, t1: QuadTree, t2: QuadTree, t3: QuadTree, t4: QuadTree) -> QuadTree {
        return QuadTree::Node {
            rect: rect,
            t1: Box::new(t1),
            t2: Box::new(t2),
            t3: Box::new(t3),
            t4: Box::new(t4),
        };
    }

    fn newLeaf(rect: Rect, points: Vec<Point>) -> QuadTree {
        assert!(points.iter().all(|p| rect.contains(p)));
        return QuadTree::Leaf {
            rect: rect,
            points: points,
        };
    }

    fn rect(&self) -> &Rect {
        return match self {
            QuadTree::Node { rect, .. } => rect,
            QuadTree::Leaf { rect, .. } => rect,
        };
    }

    fn construct(rect: Rect, points: Vec<Point>, max_size: usize) -> QuadTree {
        if points.len() <= max_size {
            return QuadTree::newLeaf(rect, points);
        }
        let xm = (rect.x1 + rect.x2) / 2;
        let ym = (rect.y1 + rect.y2) / 2;
        let r1 = Rect::new(rect.x1, rect.y1, xm - 1, ym - 1);
        let r2 = Rect::new(xm, rect.y1, rect.x2, ym - 1);
        let r3 = Rect::new(rect.x1, ym, xm - 1, rect.y2);
        let r4 = Rect::new(xm, ym, rect.x2, rect.y2);
        let p1 = points.iter().filter(|p| r1.contains(p)).cloned().collect();
        let p2 = points.iter().filter(|p| r2.contains(p)).cloned().collect();
        let p3 = points.iter().filter(|p| r3.contains(p)).cloned().collect();
        let p4 = points.iter().filter(|p| r4.contains(p)).cloned().collect();
        return QuadTree::new_node(
            rect,
            QuadTree::construct(r1, p1, max_size),
            QuadTree::construct(r2, p2, max_size),
            QuadTree::construct(r3, p3, max_size),
            QuadTree::construct(r4, p4, max_size),
        );
    }

    fn find_points(&self, filter_rect: &Rect) -> Vec<Point> {
        return match self {
            QuadTree::Node {
                rect,
                t1,
                t2,
                t3,
                t4,
            } => {
                if filter_rect.intersect(rect).is_some() {
                    union(&[
                        (*t1).find_points(filter_rect),
                        (*t2).find_points(filter_rect),
                        (*t3).find_points(filter_rect),
                        (*t4).find_points(filter_rect),
                    ])
                } else {
                    Vec::new()
                }
            }
            QuadTree::Leaf { rect, points } => points
                .iter()
                .filter(|p| filter_rect.contains(p))
                .cloned()
                .collect(),
        };
    }

    fn min_dist(&self) -> f64 {
        match self {
            QuadTree::Node {
                rect,
                t1,
                t2,
                t3,
                t4,
            } => {
                let d: f64 = *[
                    (*t1).min_dist(),
                    (*t2).min_dist(),
                    (*t3).min_dist(),
                    (*t4).min_dist(),
                ]
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
                let di = d.ceil() as i64;
                let boundary_points = union(&[
                    (*t1).find_points(&((*t2).rect()).add(di)),
                    (*t1).find_points(&((*t3).rect()).add(di)),
                    (*t2).find_points(&((*t1).rect()).add(di)),
                    (*t2).find_points(&((*t4).rect()).add(di)),
                    (*t3).find_points(&((*t1).rect()).add(di)),
                    (*t3).find_points(&((*t4).rect()).add(di)),
                    (*t4).find_points(&((*t2).rect()).add(di)),
                    (*t4).find_points(&((*t3).rect()).add(di)),
                ]);
                let d2 = min_dist_bf(&boundary_points);
                return if d < d2 { d } else { d2 };
            }
            QuadTree::Leaf { rect, points } => {
                return min_dist_bf(&points);
            }
        }
    }
}

fn union(arr: &[Vec<Point>]) -> Vec<Point> {
    let mut res: Vec<Point> = Vec::new();
    for points in arr {
        for p in points {
            res.push(p.clone());
        }
    }
    return res;
}

fn min_dist_bf(points: &Vec<Point>) -> f64 {
    let mut min_d = f64::INFINITY;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d = points[i].distance(&points[j]);
            if d > 0.0 && d < min_d {
                min_d = d;
            }
        }
    }
    return min_d;
}

fn main() {
    let n = 2000000;
    let mut s: Vec<i64> = Vec::new();
    s.push(290797);
    while s.len() <= 2 * n + 1 {
        s.push(s.last().unwrap().pow(2) % 50515093);
    }
    let mut points = (0..n)
        .map(|n| Point {
            x: s[2 * n],
            y: s[2 * n + 1],
        })
        .collect::<Vec<Point>>();
    points.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap()
            .then(a.y.partial_cmp(&b.y).unwrap())
    });
    let tree = QuadTree::construct(Rect::bounding_rect(&points), points, 4);
    println!("{:.9}", tree.min_dist());
}
