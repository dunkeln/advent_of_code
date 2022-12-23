use aoc_core::read_file;
use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    let mut stacks: [String; 9] = [
        String::from("NBDTVGZJ"),
        String::from("SRMDWPF"),
        String::from("VCRSZ"),
        String::from("RTJZPHG"),
        String::from("TCJNDZQF"),
        String::from("NVPWGSFM"),
        String::from("GCVBPQ"),
        String::from("ZBPN"),
        String::from("WPJ"),
    ];

    let data = read_file(05).unwrap();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let data = data
        .lines()
        .skip(10) // cuz first 10 lines are the description
        .map(|line| {
            let mut ret = (0_u8, 0, 0);
            re.captures_iter(line).for_each(|capture| {
                ret.0 = capture[1].parse::<u8>().unwrap();
                ret.1 = capture[2].parse::<u8>().unwrap();
                ret.2 = capture[3].parse::<u8>().unwrap();
            });
            ret
        })
        .collect::<Vec<(u8, u8, u8)>>();

    let mut stacks_cloned = stacks.clone();
    let mut sols = (String::new(), String::new());
    data.iter().for_each(|(turns, stack1, stack2)| {
        for _ in 0..(*turns) {
            let temp = stacks[*stack1 as usize - 1].pop().unwrap();
            stacks[*stack2 as usize - 1].push(temp);
        }
    });

    sols.0 = stacks
        .iter()
        .map(|stack| stack.as_bytes()[stack.len() - 1])
        .map(|ch| ch as char)
        .collect();

    data.iter().for_each(|(chunk, stack1, stack2)| {
        let mut temp = vec![];
        (0..(*chunk)).for_each(|_| temp.push(stacks_cloned[*stack1 as usize - 1].pop().unwrap()));
        stacks_cloned[*stack2 as usize - 1]
            .push_str(temp.iter().rev().collect::<String>().as_str());
    });

    sols.1 = stacks_cloned
        .iter()
        .map(|stack| stack.as_bytes()[stack.len() - 1])
        .map(|ch| ch as char)
        .collect();

    println!("{:?}", sols);
    Ok(())
}
