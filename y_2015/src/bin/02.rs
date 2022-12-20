use aoc_core::read_file;

fn refine_data(data: &str) -> Vec<(u64, u64, u64)> {
    data.lines()
        .map(|line| {
            let mut line = line.split("x")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            line.sort();
            (line[0], line[1], line[2])
        })
        .collect::<Vec<(u64, u64, u64)>>()
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(2).unwrap();
    let data = refine_data(data.as_str());
    let mut sol: (u64, u64) = (0, 0);
    sol.0 = data.iter().fold(0, |acc, trio| {
        acc + 2 * (trio.0 * trio.1 + trio.1 * trio.2 + trio.2 * trio.0)
            + trio.0 * trio.1
    }) as u64;
    sol.1 = data.iter().fold(0, |acc, trio| {
        acc + 2 * (trio.0 + trio.1) + trio.0 * trio.1 * trio.2
    }) as u64;

    println!("01: {}", sol.0);
    println!("02: {}", sol.1);
    Ok(())
}
