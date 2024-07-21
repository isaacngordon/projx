mod commands;

use clap::{Parser, Subcommand};
use commands::project::create_project;
use commands::template::{add_template, create_template};

#[derive(Parser, Debug)]
#[command(name = "projx", version = "1.0", about = "Project Management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand)]
    Template(TemplateCommands),
    #[command(subcommand)]
    Project(ProjectCommands),
}

#[derive(Subcommand, Debug)]
enum TemplateCommands {
    Add(AddTemplateArgs),
    Create(CreateTemplateArgs),
}

#[derive(Parser, Debug)]
struct AddTemplateArgs {
    name: String,
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long)]
    dir: Option<String>,
}

#[derive(Parser, Debug)]
struct CreateTemplateArgs {
    name: String,
}

#[derive(Subcommand, Debug)]
enum ProjectCommands {
    Create(CreateProjectArgs),
}

#[derive(Parser, Debug)]
struct CreateProjectArgs {
    #[arg(required = true)]
    template_key: String,
    #[arg(required = true)]
    project_name: String,
    #[arg(short, long)]
    destination: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Template(cmd) => match cmd {
            TemplateCommands::Add(args) => {
                add_template(&args.name, args.file.as_deref(), args.dir.as_deref());
            }
            TemplateCommands::Create(args) => {
                create_template(&args.name);
            }
        },
        Commands::Project(cmd) => match cmd {
            ProjectCommands::Create(args) => {
                create_project(
                    &args.template_key,
                    &args.project_name,
                    args.destination.as_deref(),
                );
            }
        },
    }
}