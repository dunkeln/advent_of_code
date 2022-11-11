use std::{env, fs};


pub fn read_file(day: u8) -> Result<String, std::io::Error>  {
    let file = format!("{:02}.txt", day);
    let path = env::current_dir().unwrap().join("src").join("data").join(file);
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
