mod settings;
mod http_client;
mod tasks;
mod dnd;

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
		Dnd {
			#[command(subcommand)]
			command: Option<DndCommands>,
		},
		Time {
			#[command(subcommand)]
			command: Option<TimeCommands>,
		},
}

#[derive(Subcommand)]
enum TaskCommands {
		/// Show the current task
		Current,

		/// List all tasks
		List,
}

#[derive(Subcommand)]
enum DndCommands {
		/// Show the current task
		Status,

		/// List all tasks
		Time,
}

#[derive(Subcommand)]
enum TimeCommands {
		/// Show the current task
		Current,

		/// List all tasks
		Context,
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
		Some(Commands::Dnd { command }) => {
			match command {
				Some(DndCommands::Status) => {
					let status = dnd::get_status()?;
					match status {
						Some(status) => {
							if status.do_not_disturb {
								println!("Do not disturb until {}", status.ends.unwrap());
							} else {
								println!("Not in do not disturb mode");
							}
						}
						None => {
							println!("No status");
						}
					}
				}
				Some(DndCommands::Time) => {
					let status = dnd::get_status()?;
					match status {
						Some(status) => {
							println!("{}", status.colored_time());
						}
						None => {
							println!("");
						}
					}
				}
				None => {
					println!("No dnd command");
				}
			}
		}
		Some(Commands::Time { command }) => {
			match command {
				Some(TimeCommands::Current) => {
					println!("{}", chrono::Local::now().format("%H:%M"));
				}
				Some(TimeCommands::Context) => {
					let dnd_status = dnd::get_status()?;
					match dnd_status {
						Some(status) => {
							if status.do_not_disturb {
								println!("{} {}", "".truecolor(0xFF, 0x7B, 0x00), status.colored_time());
							} else {
								println!("{} {}",	"󰥔", chrono::Local::now().format("%H:%M"));
							}
						}
						None => {
							println!("{} {}",	"󰥔", chrono::Local::now().format("%H:%M"));
						}
					}
				}
				None => {
					println!("No time command");
				}
			}
		}
		None => {
			println!("No command");
		}
	}

	Ok(())
}

