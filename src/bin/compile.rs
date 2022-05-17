use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Intcode file to compile
    filename: String,
}

fn main() {
    let args = Args::parse();

    let filename = args.filename;

    let raw_filename = filename.split('.').next().unwrap();
    let output_filename = format!("{}.bin", raw_filename);

    println!("{output_filename}");

    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut buffer = vec![];

    let _file_size = file
        .read_to_end(&mut buffer)
        .unwrap_or_else(|e| panic!("Couldn't read {}: {}", display, e));

    let string_content = match std::str::from_utf8(&buffer[..]) {
        Ok(str) => str,
        Err(e) => panic!("Couldn't read {}: {}", display, e),
    };

    let numbers = string_content
        .split(',')
        .map(|token| match token.parse::<i64>() {
            Ok(res) => res,
            Err(e) => panic!("Couldn't read {}: {}", display, e),
        })
        .collect::<Vec<_>>();

    let mut bytes = vec![];

    for number in numbers {
        let number_bytes: [u8; 8] = i64::to_le_bytes(number);

        for byte in number_bytes {
            bytes.push(byte);
        }
    }

    let output_path = Path::new(&output_filename);
    let output_display = output_path.display();

    let mut output_file = match File::create(&output_path) {
        Err(why) => panic!("Couldn't open {}: {}", output_display, why),
        Ok(file) => file,
    };

    output_file
        .write_all(&bytes[..])
        .unwrap_or_else(|e| panic!("Couldn't write to {}: {}", output_display, e));
}
