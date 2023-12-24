// All the numbers here are huge, so we use a multiplier to ensure we don't lose too much accuracy
// when dealing with smallish numbers. I guess this is why the bounds on the input data are in this
// order-of-magnitude..?
const PRECISION_FUDGE: f64 = 100000000000000.;

// This is the range of integer velocity values to search for on each axis.
const SEARCH_RANGE: std::ops::Range<i64> = -300..300;

struct Line {
    x1: f64,
    y1: f64,
    z1: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl Line {
    fn parse(s: &str) -> Line {
        let [x1, y1, z1, dx, dy, dz] = s
            .split(&[',', '@'][..])
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>()[..]
        else {
            unreachable!("Invalid input")
        };
        Line {
            x1,
            y1,
            z1,
            dx,
            dy,
            dz,
        }
    }

    fn intersection_xy(a: &Line, b: &Line) -> Option<(f64, f64)> {
        Self::intersection(a, b, |l| (l.x1, l.dx, l.y1, l.dy))
    }

    fn intersection_xz(a: &Line, b: &Line) -> Option<(f64, f64)> {
        Self::intersection(a, b, |l| (l.x1, l.dx, l.z1, l.dz))
    }

    fn intersection(
        a: &Line,
        b: &Line,
        uv: fn(&Line) -> (f64, f64, f64, f64),
    ) -> Option<(f64, f64)> {
        let (u1, adu, v1, adv) = uv(a);
        let (u3, bdu, v3, bdv) = uv(b);
        let u2 = u1 + PRECISION_FUDGE * adu;
        let v2 = v1 + PRECISION_FUDGE * adv;
        let u4 = u3 + PRECISION_FUDGE * bdu;
        let v4 = v3 + PRECISION_FUDGE * bdv;
        let det = (u1 - u2) * (v3 - v4) - (v1 - v2) * (u3 - u4);
        if det == 0. {
            return None;
        }
        let u = ((u1 * v2 - v1 * u2) * (u3 - u4) - (u1 - u2) * (u3 * v4 - v3 * u4)) / det;
        let v = ((u1 * v2 - v1 * u2) * (v3 - v4) - (v1 - v2) * (u3 * v4 - v3 * u4)) / det;
        if (u - u1) / adu >= 0.
            && (u - u3) / bdu >= 0.
            && (v - v1) / adv >= 0.
            && (v - v3) / bdv >= 0.
        {
            return Some((u, v));
        }
        None
    }

    fn intersects_xy(a: &Line, b: &Line) -> bool {
        Self::intersects(a, b, |l| (l.x1, l.dx, l.y1, l.dy))
    }

    fn intersects_xz(a: &Line, b: &Line) -> bool {
        Self::intersects(a, b, |l| (l.x1, l.dx, l.z1, l.dz))
    }

    fn intersects(a: &Line, b: &Line, uv: fn(&Line) -> (f64, f64, f64, f64)) -> bool {
        let (u1, adu, v1, adv) = uv(a);
        let (u3, bdu, v3, bdv) = uv(b);
        let u2 = u1 + PRECISION_FUDGE * adu;
        let v2 = v1 + PRECISION_FUDGE * adv;
        let u4 = u3 + PRECISION_FUDGE * bdu;
        let v4 = v3 + PRECISION_FUDGE * bdv;
        let det = (u1 - u2) * (v3 - v4) - (v1 - v2) * (u3 - u4);
        if det == 0. {
            // TODO: handle parallel lines properly.
            return true;
        }
        let u = ((u1 * v2 - v1 * u2) * (u3 - u4) - (u1 - u2) * (u3 * v4 - v3 * u4)) / det;
        return (u - u1) / adu >= 0. && (u - u3) / bdu >= 0.;
    }

    fn offset(&self, ox: i64, oy: i64, oz: i64) -> Line {
        let (x1, y1, z1, dx, dy, dz) = (
            self.x1,
            self.y1,
            self.z1,
            self.dx - ox as f64,
            self.dy - oy as f64,
            self.dz - oz as f64,
        );
        Line {
            x1,
            y1,
            z1,
            dx,
            dy,
            dz,
        }
    }
}

fn part1(input: &str, bounds: (f64, f64)) -> usize {
    let lines: Vec<Line> = input
        .trim()
        .split("\n")
        .map(|s| Line::parse(s /*, bounds*/))
        .collect();
    let mut total = 0;
    for i in 0..lines.len() {
        for j in i..lines.len() {
            if let Some((x, y)) = Line::intersection_xy(&lines[i], &lines[j]) {
                if x >= bounds.0 && x <= bounds.1 && y >= bounds.0 && y <= bounds.1 {
                    total += 1;
                }
            }
        }
    }
    total
}

fn part2(input: &str) -> i64 {
    let lines: Vec<Line> = input.trim().split("\n").map(|s| Line::parse(s)).collect();

    // Find a vx, vy offset pair where _all_ lines intersect in the XY plane.
    for vx in SEARCH_RANGE {
        'a: for vy in SEARCH_RANGE {
            let offset_lines = lines
                .iter()
                .map(|l| l.offset(vx, vy, 0))
                .collect::<Vec<_>>();
            for i in 0..lines.len() {
                if offset_lines[i].dx == vx as f64 || offset_lines[i].dy == vy as f64 {
                    continue;
                }
                for j in i..lines.len() {
                    if offset_lines[j].dx == vx as f64 || offset_lines[j].dy == vy as f64 {
                        continue;
                    }
                    if !Line::intersects_xy(&offset_lines[i], &offset_lines[j]) {
                        continue 'a;
                    }
                }
            }
            // Now do the same thing again, but searching for vx, vz.
            'b: for vz in SEARCH_RANGE {
                let offset_lines = lines
                    .iter()
                    .map(|l| l.offset(vx, 0, vz))
                    .collect::<Vec<_>>();
                for i in 0..lines.len() {
                    if offset_lines[i].dx == vx as f64 || offset_lines[i].dz == vz as f64 {
                        continue;
                    }
                    for j in i..lines.len() {
                        if offset_lines[j].dx == vx as f64 || offset_lines[j].dz == vz as f64 {
                            continue;
                        }
                        if !Line::intersects_xz(&offset_lines[i], &offset_lines[j]) {
                            continue 'b;
                        }
                    }
                }

                // We've found the correct velocity, now find the point of intersection on each
                // plane to find our initial position.
                let (x, y) =
                    Line::intersection_xy(&lines[0].offset(vx, vy, 0), &lines[1].offset(vx, vy, 0))
                        .unwrap();
                let (_, z) =
                    Line::intersection_xz(&lines[0].offset(vx, 0, vz), &lines[1].offset(vx, 0, vz))
                        .unwrap();

                // Rounding here is tricky, but it works OK with the input given.
                return (x.round() + y.round() + z.round()) as i64;
            }
        }
    }
    unreachable!("No solution found");
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!(
        "Part 1: {}",
        part1(input, (200000000000000., 400000000000000.))
    );
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use crate::{part1, Line};

    const TEST_INPUT: &str = include_str!("../data/example.txt");
    const TEST_BOUNDS: (f64, f64) = (7., 27.);

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(TEST_INPUT, TEST_BOUNDS));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            Some((14.333333333333334, 15.333333333333334)),
            Line::intersection_xy(
                &Line::parse("19, 13, 30 @ -2, 1, -2"),
                &Line::parse("18, 19, 22 @ -1, -1, -2"),
            )
        );
    }
}
