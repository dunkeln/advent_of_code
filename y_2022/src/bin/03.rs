use aoc_core::read_file;

fn main() -> Result<(), std::io::Error> {
    let data = read_file(03).unwrap();
    let mut sols = (0, 0);

    let data: Vec<&[u8]> = data.lines().map(str::as_bytes).collect();

    sols.0 = data.iter().fold(0, |acc, line| {
        let mut common = 0;
        let mid = line.len() >> 1;

        // TODO: make more ergonomic
        for idx in 0..mid {
            for jdx in mid..line.len() {
                if line[idx] == line[jdx] {
                    common = line[idx];
                    break;
                }
            }
            if common != 0 {
                break;
            }
        }

        acc + (match common {
            97..=122 => common + 1 - 97,
            65..=90 => common + 27 - 65,
            _ => panic!(),
        }) as u64
    });

    sols.1 = data
        .chunks(3)
        .map(|chunk| {
            let mut temp = vec![];
            chunk[0]
                .iter()
                .for_each(|x| {
                    chunk[1]
                        .iter()
                        .for_each(|y| {
                            if y == x {
                                temp.push(y);
                            }
                        })
                });
            let mut ret = 0;
            temp.iter().for_each(|&x| {
                if chunk[2].iter().any(|y| y == x) {
                    ret = *x;
                }
            });
            (match ret {
                97..=122 => ret + 1 - 97,
                65..=90 => ret + 27 - 65,
                _ => panic!(),
            }) as u64
        })
        .sum();

    println!("{:?}", sols);
    Ok(())
}
