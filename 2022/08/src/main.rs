use std::env;
use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Pos {
    x: usize,
    y: usize,
}

struct Search {
    pos: Pos,
    dir: Direction,
}

struct Grid {
    grid: Vec<Vec<i8>>,
}

impl Grid {
    fn load() -> Grid {
        let mut grid: Vec<Vec<i8>> = vec![];
        let lines = io::stdin().lines();
        for line in lines {
            let line_str = line.unwrap();
            grid.push(
                line_str.chars()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect());
        }
        return Grid { grid };
    }

    fn width(&self) -> usize {
        return self.grid[0].len();
    }

    fn height(&self) -> usize {
        return self.grid.len();
    }

    fn get(&self, pos: &Pos) -> i8 {
        return self.grid[pos.y][pos.x];
    }

    fn count_visible(&self) -> i32 {
        let mut visible = vec![vec![false; self.width()]; self.height()];
        let mut count = 0;
        for search in self.searches() {
            let mut max: i8 = -1;
            let mut pos = Some(search.pos);
            while pos.is_some() {
                let p = pos.unwrap();
                if self.get(&p) > max {
                    max = self.get(&p);
                    if visible[p.y][p.x] == false {
                        count += 1;
                        visible[p.y][p.x] = true;
                    }
                }
                pos = self.next_pos(&p, &search.dir);
            }
        }
        return count;
    }

    fn searches(&self) -> Vec<Search> {
        let (w, h) = (self.width(), self.height());
        let mut ret = vec![];
        for x in 0..w {
            ret.push(Search { pos: Pos {x, y: 0 }, dir: Direction::Down});
            ret.push(Search { pos: Pos {x, y: h-1 }, dir: Direction::Up});
        }
        for y in 0..h {
            ret.push(Search { pos: Pos { x: 0, y }, dir: Direction::Right });
            ret.push(Search { pos: Pos { x: w-1, y }, dir: Direction::Left });
        }
        return ret;
    }

    fn best_scenic_score(&self) -> i32 {
        let mut max = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos { x, y };
                let count = vec![
                    Direction::Up, Direction::Left, Direction::Down, Direction::Right,
                ].iter()
                    .map(|dir| self.count_scenic(&pos, &dir))
                    .reduce(|a, c| a * c)
                    .unwrap();
                if count > max {
                    max = count;
                }
            }
        }
        return max;
    }

    fn count_scenic(&self, pos: &Pos, dir: &Direction) -> i32 {
        let max = self.get(&pos);
        let mut count = 0;
        let mut p = Pos { x: pos.x, y: pos.y };
        while let Some(next) = self.next_pos(&p, &dir) {
            count += 1;
            if self.get(&next) >= max {
                break;
            }
            p = next;
        }
        return count;
    }

    fn next_pos(&self, pos: &Pos, dir: &Direction) -> Option<Pos> {
        let Pos { x, y } = *pos;
        let (w, h) = (self.width(), self.height());
        return match dir {
            Direction::Down => if y < h-1 { Some(Pos { x, y: y + 1 }) } else { None },
            Direction::Up => if y > 0 { Some(Pos { x, y: y - 1 }) } else { None },
            Direction::Left => if x > 0 { Some(Pos { x: x - 1, y }) } else { None },
            Direction::Right => if x < w-1 { Some(Pos { x: x + 1, y }) } else { None },
        }
    }
}

fn main() {
    let grid = Grid::load();

    if env::args().any(|x| x == "scenic") {
        print!("best scenic score: {}\n", grid.best_scenic_score());
    } else {
        print!("{} visible trees\n", grid.count_visible());
    }
}

