use std::collections::HashMap;
use aoc_core::read_file;

struct Circuit {
    registers: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            registers: HashMap::new()
        }
    }

    fn exec(&mut self, cmd: &Vec<&str>) -> u16 {
        let val = match cmd[0].parse::<u16>() {
            Ok(x) => {
                if cmd.len() == 2  {
                    return x;
                }
                let op2 = match cmd[2].parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => *self.registers.get(cmd[2]).unwrap_or(&0),
                };
                return match cmd[1] {
                    "AND" => x & op2,
                    "OR" => x | op2,
                    "LSHIFT" => x << op2,
                    "RSHIFT" => x >> op2,
                    _ => panic!("parse error")
                };
            },
            Err(_) => {
                if cmd.len() == 2 {
                    return *self.registers.get(cmd[0]).unwrap_or(&0);
                }
                match cmd[0] {
                    "NOT" => match cmd[1].parse::<u16>() {
                        Ok(num) => !num,
                        Err(_) => !*self.registers.get(cmd[1]).unwrap_or(&0)
                    },
                    op1 => {
                        let op1 = *self.registers.get(op1).unwrap_or(&0); 
                        let op2 = match cmd[2].parse::<u16>() {
                            Ok(x) => x,
                            Err(_) => *self.registers.get(cmd[2]).unwrap_or(&0),
                        };
                        return match cmd[1] {
                            "AND" => op1 & op2,
                            "OR" => op1 | op2,
                            "LSHIFT" => op1 << op2,
                            "RSHIFT" => op1 >> op2,
                            _ => panic!("parse error")
                        };
                    }
                }
            }
        };
        val
    }

    fn instruction(&mut self, cmd: &str) {
        let cmd = cmd
            .split(&[' ', '-', '>'])
            .filter(|x| x != &"")
            .collect::<Vec<&str>>();


        let out = self.exec(&cmd);
        println!("{:?}", self.registers);
        // self.registers
        //     .insert(cmd.last().unwrap().to_string(), out);
        *self.registers.get_mut(&cmd.last().unwrap().to_string()).unwrap() = out;
    }
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(07).unwrap();
    let mut circuit = Circuit::new();
    data.lines().for_each(|line| circuit.instruction(line));

    let mut sols: (u16, u16) = (0, 0);
    sols.0 = *circuit.registers.get(&"a".to_string()).unwrap();
    println!("solution 0: {}", sols.0);
    Ok(())
}

#[test]
fn get_value() {
    let data = read_file(07).unwrap();
    let data = data
        .split('\n')
        .collect::<Vec<&str>>();
    let mut circuit = Circuit::new();
    (0..180).for_each(|x| circuit.instruction(data[x]));
    println!("{:?}", circuit.registers);
}
