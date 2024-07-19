// src/commands/add_template.rs
use clap::ArgMatches;
use std::fs;
use std::path::PathBuf;
use dirs;

/// Adds a new template based on the provided arguments.
/// 
/// # Arguments
/// 
/// * `matches` - A reference to the ArgMatches containing the command line arguments.
pub fn add_template(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let file = matches.get_one::<String>("file");
    let dir = matches.get_one::<String>("dir");

    // Validate that either a file or a directory is provided, but not both.
    match (file, dir) {
        (Some(_), Some(_)) => {
            eprintln!("Error: Provide either a file or a directory, not both.");
            std::process::exit(1);
        }
        (None, None) => {
            // If no file or directory is provided, prompt the user to confirm using the current directory.
            let current_dir = std::env::current_dir().unwrap();
            println!("No file or directory provided. Are you sure you want to turn the current directory ({}) into a template? (y/n)", current_dir.display());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() != "y" {
                println!("Operation cancelled.");
                std::process::exit(1);
            }
            println!("Adding template \"{}\" using current directory: {}", name, current_dir.display());
        }
        (Some(file), None) => {
            println!("Adding template \"{}\" using file: {}", name, file);
        }
        (None, Some(dir)) => {
            println!("Adding template \"{}\" using directory: {}", name, dir);
        }
    }

    // Determine the path to the templates directory based on the build configuration.
    let path_to_templates = if cfg!(debug_assertions) {
        PathBuf::from("src/templates")
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(".projx/templates");
        home_dir
    };

    // Create the templates directory if it does not exist.
    if !path_to_templates.exists() {
        fs::create_dir_all(&path_to_templates).unwrap();
        println!("Created templates directory at: {}", path_to_templates.display());
    }

    // Create the template directory and projx.toml file if they do not exist.
    let template_dir = path_to_templates.join(name);
    let projx_toml = template_dir.join("projx.toml");

    // Check if the template directory and projx.toml file already exist.
    if template_dir.exists() && projx_toml.exists() {
        eprintln!("Error: Template directory already exists and contains a projx.toml file.");
        std::process::exit(1);
    } else if !template_dir.exists() {
        // Create the template directory and projx.toml file if the directory does not exist.
        fs::create_dir(&template_dir).unwrap();
        println!("Created template directory at: {}", template_dir.display());
        let mut file = fs::File::create(&projx_toml).unwrap();
        use std::io::Write;
        writeln!(file, "[preinstall]\ncommands = []\n").unwrap();
        writeln!(file, "[install]\ncommands = []\n").unwrap();
        writeln!(file, "[start]\ncommands = []\n").unwrap();
        writeln!(file, "[build]\ncommands = []\n").unwrap();
        writeln!(file, "[deploy]\ncommands = []\n").unwrap();
        println!("Created projx.toml file at: {}", projx_toml.display());
    } else if !projx_toml.exists() {
        // Create the projx.toml file if the template directory exists but the file does not.
        let mut file = fs::File::create(&projx_toml).unwrap();
        use std::io::Write;
        writeln!(file, "[preinstall]\ncommands = []\n").unwrap();
        writeln!(file, "[install]\ncommands = []\n").unwrap();
        writeln!(file, "[start]\ncommands = []\n").unwrap();
        writeln!(file, "[build]\ncommands = []\n").unwrap();
        writeln!(file, "[deploy]\ncommands = []\n").unwrap();
        println!("Created projx.toml file at: {}", projx_toml.display());
    }
}

