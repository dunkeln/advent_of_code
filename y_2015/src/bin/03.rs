use aoc_core::read_file;
use std::collections::HashSet;


fn change(ch: char, lokation: &mut (i8, i8)) {
    match ch {
        '>' => lokation.0 += 1,
        '<' => lokation.0 -= 1,
        '^' => lokation.1 += 1,
        'v' => lokation.1 -= 1,
        _  => panic!("invalid character supplied")
    }
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(3).unwrap();
    let mut homes: HashSet<(i8, i8)> = HashSet::new();
    let mut lokation = (0_i8, 0);
    homes.insert(lokation);

    data.trim().chars().for_each(|x| {
        change(x, &mut lokation);
        homes.insert(lokation);
    });

    let mut sol = (0_i16, 0_i16);
    sol.0 = homes.len() as i16;
    
    homes.clear();
    lokation = (0, 0);
    let mut location = (0_i8, 0);
    homes.insert(lokation);
    
    data.trim().chars().enumerate().for_each(|(i, x)| {
        if i % 2 == 0 {
            change(x, &mut lokation);
            homes.insert(lokation);
        } else {
            change(x, &mut location);
            homes.insert(location);
        }
    });

    sol.1 = homes.len() as i16;

    println!("01: {}", sol.0);
    println!("02: {}", sol.1);
    Ok(())
}
