mod file_io;

use std::fs::File;

fn main() {
    let filename = "demo.txt";
    let data = "First Line\nSecond Line\nThird Line";

    if let Err(e) = file_io::write_to_file(filename, data) {
        eprintln!("Error writing: {}", e);
        return;
    }

    match File::open(filename) {
        Ok(mut file) => {
            match file_io::count_lines(&mut file) {
                Ok(count) => println!("Line count: {}", count),
                Err(e) => eprintln!("Error counting: {}", e),
            }
        }
        Err(e) => eprintln!("Error opening file: {}", e),
    }

    match file_io::read_from_file(filename) {
        Ok(content) => println!("File contents:\n{}", content),
        Err(e) => eprintln!("Error reading: {}", e),
    }
}