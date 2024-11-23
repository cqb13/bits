use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Input in the following format: bits ./file-path");
        return;
    }

    if args.contains(&"-h".to_string()) || args.contains(&"help".to_string()) {
        println!("bits ./file-path");
        return;
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        eprintln!("Path does not exist");
        return;
    }

    if path.is_file() {
        match file_size(path) {
            Ok(size) => println!("{}", bytes_to_best_size(size)),
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        }
    } else if path.is_dir() {
        match directory_size(path) {
            Ok(size) => {
                println!("{}", bytes_to_best_size(size))
            }
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        }
    }
}

fn directory_size(path: &Path) -> Result<u64, String> {
    let items = match fs::read_dir(path) {
        Ok(items) => items,
        Err(err) => return Err(format!("Failed to read directory: {}", err)),
    };

    let mut size: u64 = 0;

    for item in items {
        let item = match item {
            Ok(item) => item,
            Err(_) => continue,
        };

        let path = item.path();
        if path.is_dir() {
            match directory_size(path.as_path()) {
                Ok(bytes) => size += bytes,
                Err(_) => continue,
            }
        } else {
            match file_size(path.as_path()) {
                Ok(bytes) => size += bytes,
                Err(_) => continue,
            }
        }
    }

    return Ok(size);
}

fn file_size(path: &Path) -> Result<u64, String> {
    match fs::read(path) {
        Ok(bytes) => return Ok(bytes.len() as u64),
        Err(err) => return Err(format!("Failed to read file: {}", err)),
    }
}

fn bytes_to_best_size(bytes: u64) -> String {
    let mut size = bytes as f64;
    let mut unit = "B";

    if size > 1024.0 {
        size /= 1024.0;
        unit = "KB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "MB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "GB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "TB";
    }

    format!("{:.2} {}", size, unit)
}
