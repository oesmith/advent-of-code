use std::collections::HashSet;
use std::io;

struct Image {
    points: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Image {
    fn parse() -> Image {
        let points: Vec<Vec<char>> = io::stdin()
                .lines()
                .map(|l| l.unwrap().chars().collect())
                .collect();
        let (width, height) = (points[0].len(), points.len());
        Image { points, width, height }
    }

    fn empty_rows(&self) -> HashSet<usize> {
        (0..self.height)
            .into_iter()
            .filter(|r| self.points[*r].iter().all(|c| *c == '.'))
            .collect()
    }

    fn empty_columns(&self) -> HashSet<usize> {
        (0..self.width)
            .into_iter()
            .filter(|c| self.points.iter().all(|r| r[*c] == '.'))
            .collect()
    }

    fn stars(&self) -> Vec<(usize, usize)> {
        let mut stars = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.points[y][x] == '#' {
                    stars.push((x, y));
                }
            }
        }
        stars
    }

    fn calculate_paths(&self) -> (usize, usize) {
        let empty_rows = self.empty_rows();
        let empty_columns = self.empty_columns();
        let stars = self.stars();

        let mut path_length = 0;
        let mut expanded_steps = 0;

        for i in 0..stars.len() {
            for j in (i + 1)..stars.len() {
                let (p1, p2) = (stars[i], stars[j]);
                let (x1, x2, y1, y2) = (
                    p1.0.min(p2.0),
                    p1.0.max(p2.0),
                    p1.1.min(p2.1),
                    p1.1.max(p2.1),
                );
                path_length += (x2 - x1)
                    + (y2 - y1);
                expanded_steps +=
                    (y1..y2)
                        .into_iter()
                        .filter(|r| empty_rows.contains(r))
                        .count()
                    + (x1..x2)
                        .into_iter()
                        .filter(|c| empty_columns.contains(c))
                        .count();
            }
        }

        (path_length, expanded_steps)
    }
}

fn main() {
    let (l, s) = Image::parse().calculate_paths();
    println!("Total path length (mul: 1): {}", l + s);
    println!( "Total path length (mul: 1M): {}", l + s * 999_999);
}
