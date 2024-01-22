use std::io::stdin;

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
    println!("{:?}", gamma * eps)
}
