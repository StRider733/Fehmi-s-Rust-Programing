use std::env;

#[derive(Debug)]

struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(size: f64, unit: &str) -> Self {
        let bytes = match unit.to_lowercase().as_str() {
            "b" | "bytes" => size,
            "kb" | "kilobytes" => size * 1000.0,
            "mb" | "megabytes" => size * 1_000_000.0,
            "gb" | "gigabytes" => size * 1_000_000_000.0,
            _ => 0.0, // default to bytes if unknown unit
        };

        Sizes {
            bytes: format!("{} bytes", bytes),
            kilobytes: format!("{} kilobytes", bytes / 1000.0),
            megabytes: format!("{} megabytes", bytes / 1_000_000.0),
            gigabytes: format!("{} gigabytes", bytes / 1_000_000_000.0),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <size> <unit>", args[0]);
        eprintln!("Example: {} \"24 mb\"", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() != 2 {
        eprintln!("Invalid format. Please use format like \"24 mb\"");
        std::process::exit(1);
    }

    let size: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number: {}", parts[0]);
            std::process::exit(1);
        }
    };

    let unit = parts[1];
    let sizes = Sizes::new(size, unit);
    
    println!("{:#?}", sizes);
}