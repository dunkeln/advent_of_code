use ::y_2015::read_file;

fn floor(ch: char, mut level: i16) -> i16 {
    match ch {
        '(' => level += 1,
        ')' => level -= 1,
        _ => panic!("invalid character supplied"),
    };
    level
}

fn first_basement(brackets: &str) -> usize {
    let mut level = 0;
    let mut iter = brackets.chars().enumerate();
    loop {
        let current = iter.next().unwrap();
        level = floor(current.1, level);
        if level == -1 {
            return current.0;
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let input = read_file(1).unwrap();
    let mut sol: (i16, i16) = (0, 0);
    sol.0 = input.trim().chars().fold(0, |acc, ch| match ch {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!("invalid character supplied"),
    });
    sol.1 = first_basement(input.as_str()) as i16 + 1;   // +1 for enumeration starting with 0
    println!("01: {}", sol.0);
    println!("02: {}", sol.1);
    Ok(())
}
