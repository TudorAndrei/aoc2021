use std::{collections::HashMap, io::stdin};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let fish = line
        .split(",")
        .map(|t| t.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let result = solve::<80>(&fish);
    println!("part 1: {result}");

    let result = solve::<256>(&fish);
    println!("part 2: {result}");
}

fn solve<const DAYS: i16>(school: &Vec<u8>) -> u64 {
    let mut cache = HashMap::<i16, u64>::new();

    let mut population = 0u64;
    for fish_days in school {
        let days = DAYS - *fish_days as i16;
        population += compute_population(days, &mut cache);
    }

    population
}

// assumes the fish has timer = 0
fn compute_population(days: i16, cache: &mut HashMap<i16, u64>) -> u64 {
    if let Some(pop) = cache.get(&days) {
        return *pop;
    }

    let pop = if days <= 0 {
        1
    } else {
        compute_population(days - 7, cache) + compute_population(days - 9, cache)
    };

    cache.insert(days, pop);

    pop
}
