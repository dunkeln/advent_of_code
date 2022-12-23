// FUCKED
// have to implement tree
use aoc_core::read_file;
use regex::Regex;

struct FileSystem {
    file_size: u8,
    left: Option<Box<FileSystem>>,
    right: Option<Box<FileSystem>>,
}

impl FileSystem {
    fn new(value: u8) -> Self {
        Self { 
            file_size: value,
            left: None,
            right: None
        }
    }
}


fn main() -> Result<(), std::io::Error> {
    let data = read_file(07);
    Ok(())
}

fn dir_size(dir: string) -> u32 {
    
}

#[test]
fn use_stack_dumass() {
    let data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let file_stack = 
    data
        .lines()

}
