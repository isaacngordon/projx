// src/commands/add_template.rs
use clap::ArgMatches;
use dirs;
use whoami;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;


/// Adds a new template based on the provided arguments.
///
/// # Arguments
///
/// * `matches` - A reference to the ArgMatches containing the command line arguments.
pub fn add_template(name: &str, file: Option<&str>, dir: Option<&str>) {
    // Validate that either a file or a directory is provided, but not both.
    // If both are provided, print an error message and exit.
    let files_to_copy: Vec<PathBuf> = match (file, dir) {
        (Some(_), Some(_)) => {
            eprintln!("Error: Provide either a file or a directory, not both.");
            std::process::exit(1);
        }
        (None, None) => {
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
            println!(
                "Adding template \"{}\" using current directory: {}",
                name,
                current_dir.display()
            );
            // crawl the directory and add all files to the files_to_copy vector
            let mut files = Vec::new();
            crawl_directory(&current_dir, &mut files);
            files
        }
        (Some(file), None) => {
            // If a file is provided, use it as the template.
            println!("Adding template \"{}\" using file: {}", name, file);
            println!("Note: Using a file as a template is not yet supported.");
            let mut files = Vec::new();
            files.push(PathBuf::from(file));
            files
        }
        (None, Some(dir)) => {
            // If a directory is provided, use it as the template.
            println!("Adding template \"{}\" using directory: {}", name, dir);
            // crawl the directory and add all files to the files_to_copy vector
            let mut files = Vec::new();
            let dir = PathBuf::from(dir);
            crawl_directory(&dir, &mut files);
            files
        }
    };
    println!("Template will consist of {} files.", files_to_copy.len());
    println!("Files to copy: {:?}", files_to_copy);

    // Determine the path to the templates directory based on the build configuration.
    let path_to_templates= get_path_to_templates();

    // Create the template directory and projx.toml file if they do not exist.
    let template_dir = path_to_templates.join(name);
    let projx_toml = template_dir.join("projx.toml");

    if !template_dir.exists() {
        // Create the template directory and projx.toml file if the directory does not exist.
        fs::create_dir(&template_dir).unwrap();
        println!("Created template directory at: {}", template_dir.display());
    }

    // Check if the template directory and projx.toml file already exist.
    if projx_toml.exists() {
        eprintln!(
            "Error: Template directory `{}` already contains a projx.toml file.",
            template_dir.display()
        );
        std::process::exit(1);
    }

    // Prompt the user for the description.
    let description = prompt_description("the template".to_string());

    // Prompt the user for the author.
    let author = prompt_author("template");

    // Create the projx.toml file and write the metadata.
    write_projx_toml_file(&projx_toml, &name, &description, &author);
    copy_files_to_template(&files_to_copy, &template_dir);

}

/// Creates a new template based on prompts to the user and with an LLM.
pub fn create_template(name: &str) {
    println!("create template command executed with name: {}", name);
}

/// Function to crawl a directory and add all files to a vector.
fn crawl_directory(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            crawl_directory(&path, files);
        } else {
            files.push(path);
        }
    }
}

fn copy_files_to_template(files: &Vec<PathBuf>, template_dir: &PathBuf) {
    for (index, file) in files.iter().enumerate() {
        let relative_path = if files.len() == 1 {
            file.file_name().unwrap()
        } else {
            file.strip_prefix(&files[0].ancestors().nth(2).unwrap()).unwrap()
        };
        let dest = template_dir.join(relative_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        println!("[{} of {}] Copying {} to {}", index + 1, files.len(), file.display(), dest.display());
        fs::copy(file, &dest).unwrap();
        println!("[{} of {}] Copied {} to {}", index + 1, files.len(), file.display(), dest.display());
    }
}

/// Prompts the user to enter a description for the provided item.
/// 
/// # Arguments
/// 
/// * `to_describe` - A String containing the name of the item to describe.
fn prompt_description(to_describe: String) -> String {
    println!("Enter a description for the {}: ", to_describe);
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    description.trim().to_string()
}

/// Prompt the user for the author.
/// 
/// # Arguments
/// 
/// * `of_item` - A String containing the name of the item to describe.
fn prompt_author(of_item: &str) -> String {
    println!("Enter the author of the {} (optional, default is system user):", of_item);
    let mut author = String::new();
    io::stdin().read_line(&mut author).unwrap();
    let author = if author.trim().is_empty() {
        whoami::realname()
    } else {
        author.trim().to_string()
    };
    author
}

/// Function to prompt the user for commands for each section.
/// 
/// # Arguments
/// 
/// * `section_name` - A String containing the name of the section to prompt for commands.
fn prompt_command_list(command_name: &str) -> Vec<String> {
    let mut commands = Vec::new();
    // Prompt the user to enter commands for the given section.
    println!(
        "Enter commands for [{}] section. Press Enter without typing anything to finish.",
        command_name
    );
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

/// Function to get the path to the templates directory.
/// 
/// # Returns
/// 
/// A PathBuf containing the path to the templates directory.
fn get_path_to_templates() -> PathBuf {
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
        println!(
            "Created templates directory at: {}",
            path_to_templates.display()
        );
    }
    path_to_templates
}

/// Function to write the projx.toml file for the template.
/// 
/// # Arguments
/// 
/// * `projx_toml` - A reference to a PathBuf containing the path to the projx.toml file.
/// * `name` - A String containing the name of the template.
/// * `description` - A String containing the description of the template.
/// * `author` - A String containing the author of the template.
fn write_projx_toml_file(projx_toml: &PathBuf, name: &str, description: &str, author: &str) {
    let mut file = fs::File::create(&projx_toml).unwrap();
    writeln!(file, "name = \"{}\"", name).unwrap();
    writeln!(file, "description = \"{}\"", description).unwrap();
    writeln!(file, "version = \"1.0.0\"").unwrap();
    writeln!(file, "author = \"{}\"\n", author).unwrap();

    let sections = ["preinstall", "install", "start", "build", "deploy"];
    for section in &sections {
        // Get the list of commands for the current section.
        let commands = prompt_command_list(section);
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
}

