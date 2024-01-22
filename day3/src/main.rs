use std::io::stdin;

fn gamma(freq: &Vec<usize>, col: usize) -> u8 {
    freq.iter()
        .map(|x| if *x > (freq.len() / 2) { b'1' } else { b'0' })
        .collect::<Vec<_>>()[col]
}
fn get_most_common_bit(lines: &Vec<String>, col: usize, mode: &str) -> u8 {
    let ones = lines
        .iter()
        .fold(vec![0; lines[0].len()], |mut freq, line| {
            for (i, x) in line.as_bytes().iter().enumerate() {
                freq[i] += if *x == b'0' { 0 } else { 1 };
            }
            freq
        })[col];
    let zeros = lines.len() - ones;

    match mode {
        "ox" => {
            if ones >= zeros {
                b'1'
            } else {
                b'0'
            }
        }
        "co" => {
            if ones < zeros {
                b'1'
            } else {
                b'0'
            }
        }
        _ => panic!("AUALEU"),
    }
}
fn main() {
    let lines = stdin().lines().filter_map(Result::ok).collect::<Vec<_>>();
    let result = lines
        .iter()
        .fold(vec![0; lines[0].len()], |mut freq, line| {
            for (i, x) in line.as_bytes().iter().enumerate() {
                freq[i] += if *x == b'0' { 0 } else { 1 };
            }
            freq
        });
    let gamma = result
        .iter()
        .map(|x| if *x > (lines.len() / 2) { "1" } else { "0" })
        .collect::<Vec<_>>()
        .concat();
    let eps = result
        .iter()
        .map(|x| if *x > (lines.len() / 2) { "0" } else { "1" })
        .collect::<Vec<_>>()
        .concat();
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let eps = i32::from_str_radix(&eps, 2).unwrap();
    println!("P1: {:?}", gamma * eps);
    let mut col: usize = 0;
    let mut ox_lines = lines.clone();

    let ox = loop {
        if ox_lines.len() == 1 {
            break &ox_lines[0];
        }
        let common_bit = get_most_common_bit(&ox_lines, col, "ox");
        // println!("{:?} {col}", common_bit as char);
        ox_lines = ox_lines
            .clone()
            .into_iter()
            .filter(|x| x.as_bytes()[col] == common_bit)
            .collect::<Vec<_>>();
        // println!("{:?}", ox_lines);
        col += 1;
    };

    let mut col: usize = 0;
    let mut co_lines = lines.clone();
    let co = loop {
        if co_lines.len() == 1 {
            break &co_lines[0];
        }
        let common_bit = get_most_common_bit(&co_lines, col, "co");
        // println!("{:?} {col}", common_bit as char);
        co_lines = co_lines
            .clone()
            .into_iter()
            .filter(|x| x.as_bytes()[col] == common_bit)
            .collect::<Vec<_>>();
        // println!("{:?}", co_lines);
        col += 1;
    };
    let result = i32::from_str_radix(ox, 2).unwrap() * i32::from_str_radix(co, 2).unwrap();
    println!("P2: {result:?}");
}
