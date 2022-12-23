use aoc_core::read_file;

trait CheckDuplicates {
    fn duplicates(&self) -> bool;
    fn duplicates_optim(&self) -> bool;
}

impl CheckDuplicates for &[u8] {
    fn duplicates(&self) -> bool {
        // using  a bitmap reqs more looping than comparisons
        // on a 4-length array (ie, 12 computations )
        (0..(self.len() - 1))
            .any(|idx| {
                ((idx+1)..self.len())
                    .any(|jdx| self[jdx] == self[idx]) 
            })
    }

    fn duplicates_optim(&self) -> bool {
        // 64 iterations to check
        // double-nested-for gives 182 iterations
        // but 182 iterations to modify bitmask
        // total iterations = 11648
        // contemplate....
        todo!();
    }
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file(06)
        .unwrap()
        .as_bytes()
        .iter()
        .map(|x| *x as u8)
        .collect::<Vec<u8>>();

    let mut sols = (0, 0);
    sols.0 = data
        .as_slice()
        .windows(4)
        .take_while(CheckDuplicates::duplicates)
        .count() + 4;

    sols.1 = data
        .as_slice()
        .windows(14)
        .take_while(CheckDuplicates::duplicates)
        .count() + 14;

    println!("{:?}", sols);
    Ok(())
}


#[test]
fn examples() {
    let data = "ssrt".as_bytes();
    println!("{:?}", data);
    assert_eq!(data.duplicates(), true);
    let data = "xyzw".as_bytes();
    println!("{:?}", data);
    assert_eq!(data.duplicates(), false);
}
