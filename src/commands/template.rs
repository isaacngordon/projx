// src/commands/add_template.rs
use dirs;
use whoami;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use crate::commands::{copy_files_to_destination, crawl_directory};

#[derive(Debug)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub author: String,
    pub files: Vec<PathBuf>,
    pub root: PathBuf,
}

impl Template {
    /// Writes the projx.toml file for the template.
    ///
    /// # Arguments
    ///
    /// * `projx_toml` - A reference to a PathBuf containing the path to the projx.toml file.
    pub fn write_projx_toml_file(&self, projx_toml: &PathBuf) {
        let mut file = fs::File::create(&projx_toml).unwrap();
        writeln!(file, "name = \"{}\"", self.name).unwrap();
        writeln!(file, "description = \"{}\"", self.description).unwrap();
        writeln!(file, "version = \"1.0.0\"").unwrap();
        writeln!(file, "author = \"{}\"\n", self.author).unwrap();

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
}

pub const DEBUG_TEMPLATES_PATH: &str = "src/templates";
pub const RELEASE_TEMPLATES_PATH: &str = ".projx/templates";


/// Adds a new template based on the provided arguments.
///
/// # Arguments
///
/// * `matches` - A reference to the ArgMatches containing the command line arguments.
pub fn add_template(name: &str, file: Option<&str>, dir: Option<&str>) -> Result<(), String>{
    // Validate that either a file or a directory is provided, but not both.
    // If both are provided, print an error message and exit.
    let files_to_copy: Vec<PathBuf> = match (file, dir) {
        (Some(_), Some(_)) => {
            return Err("Provide either a file or a directory, not both.".to_string());
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
                std::process::exit(0);
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

    let template_root = match (file, dir) {
        (Some(file), None) => PathBuf::from(file).ancestors().nth(1).unwrap().to_path_buf(),
        (None, Some(dir)) => PathBuf::from(dir),
        _ => std::env::current_dir().unwrap(),
    };

    let description = prompt_description("the template".to_string());
    let author = prompt_author("template");

    let template = Template {
        name: name.to_string(),
        description,
        author,
        files: files_to_copy,
        root: template_root,
    };

    println!("Template will consist of {} files.", template.files.len());
    println!("Files to copy: {:?}", template.files);
    println!("Template root: {}", template.root.display());

    let path_to_templates = get_path_to_templates();
    let template_dir = path_to_templates.join(&template.name);
    let projx_toml = template_dir.join("projx.toml");

    if !template_dir.exists() {
        fs::create_dir(&template_dir).unwrap();
        println!("Created template directory at: {}", template_dir.display());
    }

    if projx_toml.exists() {
        return Err(format!(
            "Error: Template directory `{}` already contains a projx.toml file.",
            template_dir.display()
        ));
    }

    template.write_projx_toml_file(&projx_toml);
    copy_files_to_destination(&template.files, &template.root, &template_dir);

    Ok(())
}

/// Creates a new template based on prompts to the user and with an LLM.
pub fn create_template(name: &str) -> Result<(), String> {
    let description = prompt_description("the template".to_string());
    let author = prompt_author("template");

    let template = Template {
        name: name.to_string(),
        description,
        author,
        files: Vec::new(),
        root: std::env::current_dir().unwrap(),
    };

    println!("create template command executed with name: {}", template.name);
    Ok(())
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
        PathBuf::from(DEBUG_TEMPLATES_PATH)
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(RELEASE_TEMPLATES_PATH);
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

