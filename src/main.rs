use std::{fs::OpenOptions, io::Read};

fn main() -> std::io::Result<()> {
    let mut filename = OpenOptions::new().read(true).open("sql.txt")?;
    let mut lines = String::new();
    filename.read_to_string(&mut lines)?;

    for line in lines.lines() {
        println!("{line}");
    }

    Ok(())
}
