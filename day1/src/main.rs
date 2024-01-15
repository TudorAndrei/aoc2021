use std::io::stdin;

fn main() {
    let lines: Vec<u32> = stdin()
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse().unwrap())
        .collect();
    let result = lines.windows(3).sum()).collect::Vec<u32>().windows(2).filter(|chunk| chunk[1] > chunk[0]).count();
    println!("{:?}", result)
}
