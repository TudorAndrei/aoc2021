use std::{io::stdin, usize};

fn gen_line(line: &String) -> (Point, Point) {
    let points = line
        .split(" -> ")
        .map(|tok| {
            let pair = tok
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Point {
                x: pair[0],
                y: pair[1],
            }
        })
        .collect::<Vec<_>>();
    (points[0], points[1])
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct Grid {
    points: Vec<Vec<i32>>,
}
impl Grid {
    fn fill1(&mut self, (start, end): (Point, Point)) {
        if start.x == end.x {
            let min = start.y.min(end.y);
            let max = start.y.max(end.y);
            for i in min..=max {
                self.points[i][start.x] += 1;
            }
        } else if start.y == end.y {
            let min = start.x.min(end.x);
            let max = start.x.max(end.x);
            for i in min..=max {
                self.points[start.y][i] += 1;
            }
        }
    }
    fn fill2(&mut self, (start, end): (Point, Point)) {
        if start.x == end.x {
            let min = start.y.min(end.y);
            let max = start.y.max(end.y);
            for i in min..=max {
                self.points[i][start.x] += 1;
            }
        } else if start.y == end.y {
            let min = start.x.min(end.x);
            let max = start.x.max(end.x);
            for i in min..=max {
                self.points[start.y][i] += 1;
            }
        } else if (start.x > end.x && start.y > end.y) || (start.x < end.x && start.y < end.y) {
            let min_j = start.x.min(end.x);
            let max_j = start.x.max(end.x);

            let min_i = start.y.min(end.y);

            for j in min_j..=max_j {
                let i = min_i + j - min_j;
                self.points[i][j] += 1;
            }
        } else {
            let min_j = start.x.min(end.x);
            let max_j = start.x.max(end.x);

            // let min_i = start.y.min(end.y);
            let max_i = start.y.max(end.y);

            for j in min_j..=max_j {
                let i = max_i + min_j - j;
                self.points[i][j] += 1;
            }
        }
    }
    fn new(w: usize, h: usize) -> Grid {
        Grid {
            points: vec![vec![0; w]; h],
        }
    }
    fn print(&self) {
        for i in 0..self.points.len() {
            for j in 0..self.points[0].len() {
                print!("{:?}", self.points[i][j])
            }
            println!();
        }
    }
}

fn solve1(grid: &mut Grid, lines: &Vec<(Point, Point)>) -> i32 {
    lines.iter().for_each(|line| grid.fill1(*line));
    let mut count = 0;

    for i in 0..grid.points.len() {
        for j in 0..grid.points[0].len() {
            if grid.points[i][j] > 1 {
                count += 1;
            }
        }
    }
    count
}
fn solve2(grid: &mut Grid, lines: &Vec<(Point, Point)>) -> i32 {
    lines.iter().for_each(|line| grid.fill2(*line));
    let mut count = 0;

    for i in 0..grid.points.len() {
        for j in 0..grid.points[0].len() {
            if grid.points[i][j] > 1 {
                count += 1;
            }
        }
    }
    count
}
fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(Result::ok).collect();
    let lines: Vec<(Point, Point)> = lines.iter().map(gen_line).collect();
    let (w, h) = lines.iter().fold((0, 0), |(w, h), (start, end)| {
        (w.max(start.x.max(end.x)), h.max(start.y.max(end.y)))
    });
    let mut grid = Grid::new(w + 1, h + 1);
    println!("P1: {}", solve1(&mut grid, &lines));
    let mut grid = Grid::new(w + 1, h + 1);
    println!("P2: {}", solve2(&mut grid, &lines));
}
