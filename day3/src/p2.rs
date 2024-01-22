use std::io::stdin;

fn gamma(freq: &Vec<usize>, col: u32) -> String {
    freq.iter()
        .map(|x| if *x > (freq.len() / 2) { "1" } else { "0" })
        .collect::<Vec<_>>()
        .concat()
        .to_string()
}
fn get_freq(lines: &Vec<String>, col: u32) -> Vec<i32> {
    lines
        .iter()
        .fold(vec![0; lines[0].len()], |mut freq, line| {
            for (i, x) in line.as_bytes().iter().enumerate() {
                freq[i] += if *x == b'0' { 0 } else { 1 };
            }
            freq
        })
}
fn main() {
    let lines = stdin().lines().filter_map(Result::ok).collect::<Vec<_>>();

    println!("{:?}", gamma * eps)
}
