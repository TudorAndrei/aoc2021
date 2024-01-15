use std::{collections::HashSet, io::stdin, iter::zip};

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<u32>,
    selected: Vec<bool>,
}

impl Board {
    fn parse(lines: &[String]) -> Board {
        let mut grid = Vec::<u32>::new();
        let mut selected = Vec::<bool>::new();

        for line in lines[1..].iter() {
            let mut numbers = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            grid.append(&mut numbers);
            selected.append(&mut vec![false; 5]);
        }

        Board { grid, selected }
    }

    fn update(&mut self, number: u32) {
        let i = self.grid.iter().position(|a| *a == number);

        match i {
            None => (),
            Some(i) => self.selected[i] = true,
        }
    }

    fn solved(&self) -> bool {
        // check rows
        for row in self.selected.chunks(5) {
            if !row.contains(&false) {
                return true;
            }
        }

        // check colums
        for j in 0..5 {
            let mut col = self.selected[j..].iter().step_by(5);
            if col.all(|&x| x) {
                return true;
            }
        }

        return false;
    }

    fn score(&self) -> u32 {
        zip(&self.grid, &self.selected)
            .filter(|(_, &s)| !s)
            .fold(0, |sum, (n, _)| sum + n)
    }
}

fn main() {
    let lines = stdin().lines().filter_map(Result::ok).collect::<Vec<_>>();

    let numbers = lines[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = lines[1..]
        .chunks(6)
        .map(|lines| Board::parse(&lines))
        .collect::<Vec<_>>();

    let mut boards_clone = boards.clone();

    let mut final_score = 0;
    'outer: for &n in &numbers {
        for board in boards.iter_mut() {
            board.update(n);

            if board.solved() {
                final_score = board.score() * n;
                break 'outer;
            }
        }
    }

    println!("part 1: {final_score}");

    let mut final_score = 0;
    let mut remaining_boards = (0..boards_clone.len())
        .into_iter()
        .collect::<HashSet<usize>>();

    'outer: for &n in &numbers {
        for &i in remaining_boards.clone().iter() {
            boards_clone[i].update(n);
            if boards_clone[i].solved() {
                if remaining_boards.len() == 1 {
                    final_score = boards_clone[i].score() * n;
                    break 'outer;
                } else {
                    remaining_boards.remove(&i);
                }
            }
        }
    }
    println!("part 2: {final_score}");
}
