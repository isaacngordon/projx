// src/commands/add_template.rs
use clap::ArgMatches;
use std::fs;
use std::path::PathBuf;
use dirs;

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
            println!("Added template with name: {} using current directory: {}", name, current_dir.display());
        }
        (Some(file), None) => {
            println!("Added template with name: {} using file: {}", name, file);
        }
        (None, Some(dir)) => {
            println!("Added template with name: {} using directory: {}", name, dir);
        }
    }

    let path_to_templates = if cfg!(debug_assertions) {
        PathBuf::from("src/templates")
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(".projx/templates");
        home_dir
    };

    if !path_to_templates.exists() {
        fs::create_dir_all(&path_to_templates).unwrap();
        println!("Created templates directory at: {}", path_to_templates.display());
    }

    let template_dir = path_to_templates.join(name);
    let projx_toml = template_dir.join("projx.toml");

    if template_dir.exists() && projx_toml.exists() {
        eprintln!("Error: Template directory already exists and contains a projx.toml file.");
        std::process::exit(1);
    } else if !template_dir.exists() {
        fs::create_dir(&template_dir).unwrap();
        println!("Created template directory at: {}", template_dir.display());
        fs::File::create(&projx_toml).unwrap();
        println!("Created projx.toml file at: {}", projx_toml.display());
    } else if !projx_toml.exists() {
        fs::File::create(&projx_toml).unwrap();
        println!("Created projx.toml file at: {}", projx_toml.display());
    }
}

