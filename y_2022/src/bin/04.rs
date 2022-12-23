use aoc_core::read_file;
use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let data = read_file(04)
        .unwrap()
        .lines()
        .map(|line| {
            let mut ret = (0, 0, 0, 0);
            re.captures_iter(line)
                .for_each(|capture| {
                    ret.0 = capture[1].parse::<i8>().unwrap();
                    ret.1 = capture[2].parse::<i8>().unwrap();
                    ret.2 = capture[3].parse::<i8>().unwrap();
                    ret.3 = capture[4].parse::<i8>().unwrap();
                });
            ret
        })
        .collect::<Vec<(i8, i8, i8, i8)>>();

    let mut sols = (0, 0);
    sols.0 = data
        .iter()
        .map(|line| {
            let (x, y, z, w) = line;
            if x <= z && w <= y || z <= x && y <= w {
                return true;
            }
            false
        })
        .filter(|x| *x)
        .count();

    sols.1 = data
        .iter()
        .map(|line| {
            let (x, y, z, w) = line;
            if x <= z && z <= y || x <= w && w <= y || z <= x && y <= w {
                return true;
            }
            false
        })
        .filter(|x| *x)
        .count();

    println!("{:?}", sols);
    Ok(())
}
