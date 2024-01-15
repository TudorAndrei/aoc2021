use std::io::stdin;

fn main() {
    let lines = stdin().lines().filter_map(Result::ok).collect::<Vec<_>>();

    let result = lines.iter().fold((0, 0), |(h, d), line| {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["forward", x] => (h + x.parse::<i32>().unwrap(), d),
            ["down", x] => (h, d + x.parse::<i32>().unwrap()),
            ["up", x] => (h, d - x.parse::<i32>().unwrap()),
            _ => panic!("idk"),
        }
    });

    println!("part 1: {}", result.0 * result.1);

    let result = lines.iter().fold((0, 0, 0), |(h, d, a), line| {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["forward", x] => {
                let x = x.parse::<i32>().unwrap();
                (h + x, d + a * x, a)
            }
            ["down", x] => (h, d, a + x.parse::<i32>().unwrap()),
            ["up", x] => (h, d, a - x.parse::<i32>().unwrap()),
            _ => panic!("idk"),
        }
    });

    println!("part 2: {}", result.0 * result.1);
}
