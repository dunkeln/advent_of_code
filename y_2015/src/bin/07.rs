use std::collections::HashMap;
use ::y_2015::read_file;


struct Circuit {
    registers: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            registers: HashMap::new()
        }
    }

    fn instruction(&mut self, cmd: &str) {
        let cmd = cmd
            .split(&[' ', '-', '>'])
            .filter(|x| x != &"")
            .collect::<Vec<&str>>();
        println!("{:?}", cmd);
        let to_reg = cmd.last().unwrap().to_string();

        // FINAL PIECE
        self.registers
            .entry(to_reg)
            .and_modify(|x| *x = 0)
            .or_insert(0);
    }
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(07).unwrap();
    let mut circuit = Circuit::new();
    data.lines().for_each(|line| circuit.instruction(line));
    Ok(())
}
