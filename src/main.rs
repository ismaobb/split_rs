use std::fs::{create_dir, File};
use std::io::{stdin, Read, Result, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<()> {
    println!("File splitting tool Power By Rust");

    loop {
        println!("Please enter the file to be processed:");
        let mut filenname = String::new();
        stdin().read_line(&mut filenname)?;
        filenname = filenname.trim().to_string();

        if !Path::new(&filenname).exists() {
            eprintln!("Error: No such file or directory");
            continue;
        }

        let mut file = File::open(filenname)?;
        let mut buffer = [0; 1 << 20];
        let mut chunk_count = 1;
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let output_folder = format!("{time}");

        if !Path::new(&output_folder).exists() {
            create_dir(&output_folder)?;
        }

        let start = SystemTime::now();

        loop {
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            let output_file_name = format!("{}/chunk_{}", output_folder, chunk_count);
            let mut output_file = File::create(&output_file_name)?;
            output_file.write_all(&buffer[..bytes_read])?;

            chunk_count += 1;
        }

        println!(
            "\nFile splitting complete.\nfolder {}.\ntake {} ms.\n{} chunks created.\n",
            output_folder,
            chunk_count,
            SystemTime::now().duration_since(start).unwrap().as_millis()
        );
    }
}
