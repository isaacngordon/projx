mod commands;

use clap::{Command, Arg};
use commands::create_project::create_project;
use commands::add_template::add_template;
use commands::create_template::create_template;

fn main() {
    let matches = Command::new("projx")
        .version("1.0")
        .about("Project Management CLI")
        .subcommand(
            Command::new("template")
                .about("Commands related to templates")
                .subcommand(
                    Command::new("add")
                        .about("Adds a new template")
                        .arg(Arg::new("name").required(true))
                        .arg(Arg::new("file").short('f').long("file"))
                        .arg(Arg::new("dir").short('d').long("dir"))
                        .arg(Arg::new("destination").short('d').long("destination"))
                
                )
                .subcommand(
                    Command::new("create")
                        .about("Creates a new template")
                        .arg(Arg::new("name").required(true))
                )
        )
        .subcommand(
            Command::new("project")
                .about("Commands related to projects")
                .subcommand(
                    Command::new("create")
                        .about("Creates a new project")
                        .arg(Arg::new("template-key").required(true))
                        .arg(Arg::new("project-name").required(true))
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("template", template_matches)) => {
            match template_matches.subcommand() {
                Some(("add", add_matches)) => add_template(add_matches),
                Some(("create", create_matches)) => create_template(create_matches),
                _ => eprintln!("Unknown template command"),
            }
        }
        Some(("project", project_matches)) => {
            match project_matches.subcommand() {
                Some(("create", create_matches)) => create_project(create_matches),
                _ => eprintln!("Unknown project command"),
            }
        }
        _ => eprintln!("Unknown command"),
    }
}
