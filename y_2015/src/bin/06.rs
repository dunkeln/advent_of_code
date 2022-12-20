use aoc_core::read_file;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Cmd {
    On,
    Off,
    Toggle,
}
#[derive(Debug, Copy, Clone)]
struct Details {
    x: usize,
    y: usize,
    i: usize,
    j: usize,
    cmd: Cmd,
}

// TODO: try using SIMD instructions
// TODO: O(N²) time, try cutting to O(N)
fn mutate_grid(grid: &mut [[u8; 1000]; 1000], details: Details) {
    assert!(details.x <= details.i);
    assert!(details.y <= details.j);
    match details.cmd {
        Cmd::On => (details.x..=details.i)
            .for_each(|row| (details.y..=details.j)
                .for_each(|bulb| grid[row][bulb] = 1)),
        Cmd::Off => (details.x..=details.i)
            .for_each(|row| (details.y..=details.j)
                .for_each(|bulb| grid[row][bulb] = 0)),
        Cmd::Toggle => (details.x..=details.i)
            .for_each(|row| { (details.y..=details.j)
                .for_each(|bulb| grid[row][bulb] = grid[row][bulb] ^ 1)
        }),
    };
}

fn mutate_upgraded_grid(grid: &mut [[u8; 1000]; 1000], details: Details) {
    assert!(details.x <= details.i);
    assert!(details.y <= details.j);
    match details.cmd {
        Cmd::On => (details.x..=details.i)
            .for_each(|row| (details.y..=details.j)
                .for_each(|bulb| grid[row][bulb] += 1)),
        Cmd::Off => (details.x..=details.i)
            .for_each(|row| (details.y..=details.j)
                .for_each(|bulb| if grid[row][bulb] > 0 { grid[row][bulb] -= 1 })),
        Cmd::Toggle => (details.x..=details.i)
            .for_each(|row| { (details.y..=details.j)
                .for_each(|bulb| grid[row][bulb] += 2)
        }),
    };
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(6).unwrap();
    let mut grid = [[0; 1000]; 1000];
    let mut upgraded_grid = [[0; 1000]; 1000];

    let regex = Regex::new(r"(\d+),(\d+) \w* (\d+),(\d+)").unwrap();

    // NOTE: total O(N³) time
    data
        .lines()
        .for_each(|x| {
            // PERF: tradeoff for modifying data with enum
            // cuz only namespace enum is 1 byte but &str type is bigger
            let cmd = if x.contains("off") {
                Cmd::Off
            } else if x.contains("on") {
                Cmd::On
            } else {
                Cmd::Toggle
            };
            let positions = regex.captures(&x).unwrap();

            let details = Details {
                cmd: cmd,
                x: positions
                    .get(1)
                    .map_or(1_001, |x| x.as_str().parse::<usize>().unwrap()),
                y: positions
                    .get(2)
                    .map_or(1_001, |x| x.as_str().parse::<usize>().unwrap()),
                i: positions
                    .get(3)
                    .map_or(1_001, |x| x.as_str().parse::<usize>().unwrap()),
                j: positions
                    .get(4)
                    .map_or(1_001, |x| x.as_str().parse::<usize>().unwrap()),
            };

            mutate_grid(&mut grid, details);
            mutate_upgraded_grid(&mut upgraded_grid, details);
        });

    let mut sols: (u64, u64) = (0, 0);
    sols.0 = grid
        .iter()
        .map(|row| row.iter().fold(0, |acc, x| acc + *x as u64))
        .fold(0, |acc: u64, val| acc + val);
    sols.1 = upgraded_grid
        .iter()
        .map(|row| row.iter().fold(0, |acc, x| acc + *x as u64))
        .fold(0, |acc: u64, val| acc + val);
    println!("#1: {}", sols.0);
    println!("#2: {}", sols.1);
    Ok(())
}
