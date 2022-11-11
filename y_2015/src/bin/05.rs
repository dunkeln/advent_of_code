use ::y_2015::read_file;
use regex::Regex;

// const VOWELS: &[char; 5] = &['a', 'e', 'i', 'o', 'u'];
// const RESTRICTED: &[&str; 4] = &["ab", "cd", "pq", "xy"];

fn main() -> Result<(), std::io::Error> {
    let data = read_file(5).unwrap();
    let mut sol = (0_u16, 0);

    let regex_string = r"*(aeiou){3}|^(ab|cd|pq|xy)*";
    let re = Regex::new(regex_string).unwrap();

    sol.0 = data
        .lines()
        .take_while(|line| re.is_match(line))
        .count() as u16;

    let regex_string = r"";
    let re = Regex::new(regex_string).unwrap();

    sol.1 = data
        .lines()
        .take_while(|line| re.is_match(line))
        .count() as u16;

    Ok(())
}
