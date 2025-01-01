use colored::*;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, stdin, Read, Write};

pub fn usr_input<'a>(message: &str, buffer: &'a mut String) -> &'a str {
    print!("{}", message.blue().bold()); // Styled input prompt
    io::stdout().flush().unwrap();
    stdin().read_line(buffer).unwrap();
    buffer.trim()
}

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

fn main() -> io::Result<()> {
    let mut f1_hasher = Sha256::new();
    let mut f2_hasher = Sha256::new();

    // Get the first file path
    let mut file1_path = String::new();
    file1_path = usr_input("Enter the first file path: ", &mut file1_path).to_string();
    while !file_exists(file1_path.trim()) {
        println!("{} {}", "File Not Found:".red(), file1_path.yellow().bold());
        println!("{}", "Please verify the path and try again.".italic());
        file1_path = usr_input("Enter the first file path: ", &mut file1_path).to_string();
    }

    // Get the second file path
    let mut file2_path = String::new();
    file2_path = usr_input("Enter the second file path: ", &mut file2_path).to_string();
    while !file_exists(file2_path.trim()) {
        println!("{} {}", "File Not Found:".red(), file2_path.yellow().bold());
        println!("{}", "Please verify the path and try again.".italic());
        file2_path = usr_input("Enter the second file path: ", &mut file2_path).to_string();
    }

    // Open and hash the files
    let mut file1 = fs::File::open(file1_path.trim())?;
    let mut file2 = fs::File::open(file2_path.trim())?;

    let mut read_buff = Vec::new();

    // Hash first file
    file1.read_to_end(&mut read_buff)?;
    f1_hasher.update(&read_buff);
    read_buff.clear();

    // Hash second file
    file2.read_to_end(&mut read_buff)?;
    f2_hasher.update(&read_buff);

    // Compare file hashes
    let f1_result = f1_hasher.finalize();
    let f2_result = f2_hasher.finalize();

    if f1_result == f2_result {
        println!("\n{}", "✔ Files are identical!".green().bold());
    } else {
        println!("\n{}", "✘ Files are different.".red().bold());
    }

    Ok(())
}
