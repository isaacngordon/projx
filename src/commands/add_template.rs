// src/commands/add_template.rs
use clap::ArgMatches;
use std::fs;
use std::path::PathBuf;
use dirs;
use std::io::{self, Write};

/// Adds a new template based on the provided arguments.
/// 
/// # Arguments
/// 
/// * `matches` - A reference to the ArgMatches containing the command line arguments.
pub fn add_template(matches: &ArgMatches) {
    // Retrieve the template name from the command line arguments.
    let name = matches.get_one::<String>("name").unwrap();
    let file = matches.get_one::<String>("file");
    let dir = matches.get_one::<String>("dir");

    // Validate that either a file or a directory is provided, but not both.
    // If both are provided, print an error message and exit.
    match (file, dir) {
        (Some(_), Some(_)) => {
            eprintln!("Error: Provide either a file or a directory, not both.");
            std::process::exit(1);
        }
        (None, None) => {
            // If no file or directory is provided, prompt the user to confirm using the current directory.
            // If no file or directory is provided, prompt the user to confirm using the current directory.
            let current_dir = std::env::current_dir().unwrap();
            println!("No file or directory provided. Are you sure you want to turn the current directory ({}) into a template? (y/n)", current_dir.display());
            // Read user input to confirm using the current directory.
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() != "y" {
                println!("Operation cancelled.");
                std::process::exit(1);
            }
            // If confirmed, proceed with using the current directory as the template.
            println!("Adding template \"{}\" using current directory: {}", name, current_dir.display());
        }
        (Some(file), None) => {
            // If a file is provided, use it as the template.
            println!("Adding template \"{}\" using file: {}", name, file);
        }
        (None, Some(dir)) => {
            // If a directory is provided, use it as the template.
            println!("Adding template \"{}\" using directory: {}", name, dir);
        }
    }

    // Determine the path to the templates directory based on the build configuration.
    // Determine the path to the templates directory based on the build configuration.
    let path_to_templates = if cfg!(debug_assertions) {
        PathBuf::from("src/templates")
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(".projx/templates");
        home_dir
    };

    // Create the templates directory if it does not exist.
    // Create the templates directory if it does not exist.
    if !path_to_templates.exists() {
        fs::create_dir_all(&path_to_templates).unwrap();
        println!("Created templates directory at: {}", path_to_templates.display());
    }

    // Create the template directory and projx.toml file if they do not exist.
    // Create the template directory and projx.toml file if they do not exist.
    let template_dir = path_to_templates.join(name);
    let projx_toml = template_dir.join("projx.toml");

    // Check if the template directory and projx.toml file already exist.
    // Check if the template directory and projx.toml file already exist.
    if template_dir.exists() && projx_toml.exists() {
        eprintln!("Error: Template directory already exists and contains a projx.toml file.");
        std::process::exit(1);
    } else if !template_dir.exists() {
        // Create the template directory and projx.toml file if the directory does not exist.
        // Create the template directory and projx.toml file if the directory does not exist.
        fs::create_dir(&template_dir).unwrap();
        println!("Created template directory at: {}", template_dir.display());
        // Function to prompt the user for commands for each section.
        fn get_command_list(section_name: &str) -> Vec<String> {
            let mut commands = Vec::new();
            // Prompt the user to enter commands for the given section.
            println!("Enter commands for [{}] section. Press Enter without typing anything to finish.", section_name);
            loop {
                // Read user input for commands.
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let command = input.trim().to_string();
                // Break the loop if the input is empty.
                if command.is_empty() {
                    break;
                }
                commands.push(command);
            }
            commands
        }

        // List of sections to prompt for commands.
        let sections = ["preinstall", "install", "start", "build", "deploy"];
        // Create the projx.toml file and write the commands for each section.
        // Create the projx.toml file if the template directory exists but the file does not.
        let mut file = fs::File::create(&projx_toml).unwrap();
        for section in &sections {
            // Get the list of commands for the current section.
            let commands = get_command_list(section);
            // Write the section header and commands to the projx.toml file.
            writeln!(file, "[{}]\ncommands = [", section).unwrap();
            for command in commands {
                writeln!(file, "\"{}\",", command).unwrap();
            }
            // Close the commands list for the current section.
            writeln!(file, "]\n").unwrap();
        }
        // Print a message indicating that the projx.toml file was created.
        println!("Created projx.toml file at: {}", projx_toml.display());
    } else if !projx_toml.exists() {
        // Create the projx.toml file if the template directory exists but the file does not.
        let mut file = fs::File::create(&projx_toml).unwrap();
        use std::io::Write;
        // Write empty command lists for each section.
        writeln!(file, "[preinstall]\ncommands = []\n").unwrap();
        writeln!(file, "[install]\ncommands = []\n").unwrap();
        writeln!(file, "[start]\ncommands = []\n").unwrap();
        writeln!(file, "[build]\ncommands = []\n").unwrap();
        writeln!(file, "[deploy]\ncommands = []\n").unwrap();
        println!("Created projx.toml file at: {}", projx_toml.display());
    }
}

