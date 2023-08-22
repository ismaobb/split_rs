use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    // 设置输入文件名和每个分片的大小
    let input_file = &args[1];

    let mut file = File::open(input_file).expect("Failed to open input file");
    let mut buffer = [0; 1 << 20];

    let mut chunk_count = 0;
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("{}", time);

    let output_folder = format!("{time}");
    if !Path::new(&output_folder).exists() {
        fs::create_dir(&output_folder).expect("Failed to create output folder");
    }

    loop {
        let bytes_read = file.read(&mut buffer).expect("Failed to read from file");

        if bytes_read == 0 {
            break;
        }

        let output_file_name = format!("{}/chunk_{}", output_folder, chunk_count);
        let mut output_file =
            File::create(&output_file_name).expect("Failed to create output file");
        output_file
            .write_all(&buffer[..bytes_read])
            .expect("Failed to write to output file");

        chunk_count += 1;
    }

    println!("File splitting complete. {} chunks created.", chunk_count);
}
