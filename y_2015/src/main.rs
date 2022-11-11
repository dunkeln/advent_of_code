use std::process::Command;


fn main() {
    (1..=25).for_each(|day| {
        let file = format!("{:2}.txt", day);
        Command::new("cargo")
            .args(&["run", "--release", "--bin", &file])
            .output()
            .unwrap();
    });
}
