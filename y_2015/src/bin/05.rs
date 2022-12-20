use ::y_2015::read_file;
use regex::RegexSet;

// const VOWELS: &[char; 5] = &['a', 'e', 'i', 'o', 'u'];
// const RESTRICTED: &[&str; 4] = &["ab", "cd", "pq", "xy"];
// TODO: get the right regex
// case 1: 
// + >= 3 vowels
// + no substring like 'ab', 'cd', 'pq', 'xy'
// + a letter repeated consecutively twice ONLY
//
// case 2:
// + a unique pair of letters appearing atleast twice
// + a triplet of form xyx



fn total_nice_strs(data: &str, regexs: &[&str]) -> usize {
    let re = RegexSet::new(regexs).unwrap();
    data
        .lines()
        .filter(|x| re.is_match(x))
        .collect::<Vec<&str>>()
        .len()
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(5).unwrap();
    let mut sol = (0_u16, 0);

    let condition_1 = &[
        r"\w{2}",
        r"[aeiou]{3,}",
        r"^(ab|cd|xy|pq)"
    ];

    let condition_2 = &[r""];

    sol.0 = total_nice_strs(data.as_str(), condition_1) as u16;
    sol.1 = total_nice_strs(data.as_str(), condition_2) as u16;

    println!("sol.0 = {}", sol.0);
    println!("sol.1 = {}", sol.1);
    Ok(())
}
