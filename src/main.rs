mod settings;
mod http_client;
mod tasks;

use std::error::Error;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Actions related to tasks
    Task {
    	#[command(subcommand)]
    	command: Option<TaskCommands>,
    },
}

#[derive(Subcommand)]
enum TaskCommands {
		/// Show the current task
		Current,

		/// List all tasks
		List,
}

fn main() -> Result<(), Box<dyn Error>> {

	let cli = Cli::parse();

	match &cli.command {
		Some(Commands::Task { command }) => {
			match command {
				Some(TaskCommands::Current) => {
					let current_task = tasks::get_one()?;
					match current_task.task {
						Some(task) => {
							println!("{} {}", task.colored_dot(), task.title);
						}
						None => {
							println!("{}","No tasks in the current context".green());
						}
					}
				}
				Some(TaskCommands::List) => {
					println!("List tasks");
				}
				None => {
					println!("No task command");
				}
			}
		}
		None => {
			println!("No command");
		}
	}

	Ok(())
}

