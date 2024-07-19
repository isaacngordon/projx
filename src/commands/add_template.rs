// src/commands/add_template.rs
use clap::ArgMatches;

pub fn add_template(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let file = matches.get_one::<String>("file");
    let dir = matches.get_one::<String>("dir");

    match (file, dir) {
        (Some(_), Some(_)) => {
            eprintln!("Error: Provide either a file or a directory, not both.");
            std::process::exit(1);
        }
        (None, None) => {
            let current_dir = std::env::current_dir().unwrap();
            println!("No file or directory provided. Are you sure you want to turn the current directory ({}) into a template? (y/n)", current_dir.display());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() != "y" {
                println!("Operation cancelled.");
                std::process::exit(1);
            }
            println!("add template command executed with name: {} and directory: {}", name, current_dir.display());
        }
        (Some(file), None) => {
            println!("add template command executed with name: {} and file: {}", name, file);
        }
        (None, Some(dir)) => {
            println!("add template command executed with name: {} and directory: {}", name, dir);
        }
    }
}

