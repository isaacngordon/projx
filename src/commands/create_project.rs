// src/commands/create_project.rs
use clap::ArgMatches;

pub fn create_project(matches: &ArgMatches) {
    let template_key = matches.get_one::<String>("template-key").unwrap();
    let project_name = matches.get_one::<String>("project-name").unwrap();
    // Ensure the template directory exists
    let path_to_templates = if cfg!(debug_assertions) {
        PathBuf::from("src/templates")
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(".projx/templates");
        home_dir
    };

    let template_dir = path_to_templates.join(template_key);

    if !template_dir.exists() {
        eprintln!("Error: Template directory '{}' does not exist.", template_key);
        std::process::exit(1);
    }

    println!("create project command executed with template-key: {} and project-name: {}", template_key, project_name);
}

