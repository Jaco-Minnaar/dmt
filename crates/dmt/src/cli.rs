use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Database Migration Tool (dmt)")]
#[command(author = "Jaco Minnaar <jaco@jacominnaar.com>")]
#[command(version)]
#[command(about = "A standalone database migration utility.")]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: DmtCommand,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", default_value = "./dmt.config.yml")]
    pub config: String,
}

#[derive(Subcommand)]
pub enum DmtCommand {
    /// Creates a new migration
    New(NewMigrationArgs),
    /// Execute all outstanding migrations
    Migrate,
    /// Rollback to before last migration
    Rollback,
}

#[derive(Args)]
pub struct NewMigrationArgs {
    /// The name of the new migration
    pub name: String,
}
