// file io in rust


use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

pub fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    println!("wrote to {}", filename);
    Ok(())
}

pub fn read_from_file(filename: &str)  -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn count_lines(file: &mut File) -> io::Result<usize> {
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}