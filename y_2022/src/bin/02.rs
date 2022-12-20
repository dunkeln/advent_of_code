use aoc_core::read_file;

fn main() -> Result<(), std::io::Error> {
    let data = read_file(02).unwrap();
    let mut sols = (usize::MIN, usize::MIN);
    let data = data
        .lines()
        .map(str::as_bytes)
        .map(|line| (line[0] - b'A', line[2] - b'X'));
    sols.0 = data
        .clone()
        .fold(0, |acc, (opp, player)| 
            (match (opp, player) {
                (0, 2) | (1, 0) | (2, 1) => 0,
                (0, 0) | (1, 1) | (2, 2) => 3,
                (0, 1) | (1, 2) | (2, 0) => 6,
                _ => unreachable!()
            } + player
                + 1) as usize + acc
        );

    sols.1 = data
        .fold(0, |acc, (opp, score)| acc + ((opp + (score + 2) % 3) % 3 + 1 + score * 3) as usize);

    println!("{:?}", sols);
    Ok(())
}
