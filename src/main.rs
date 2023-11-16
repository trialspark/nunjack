use std::path::PathBuf;

use clap::{Parser, Subcommand};
use nunjack_core::{error::NunjackResult, Check, CheckStatus, ExecCheck, PidCheck, SocketCheck};

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    /// Check for a currently running process with id PID
    Pid {
        /// Process ID number
        pid: i32,
    },

    /// Check for a currently running process with id from PATH
    PidFile {
        /// Path to a file which contains a PID (and optional trailing whitespace)
        path: PathBuf,
    },

    /// Check for socket connectivity
    Socket { sock: String },

    /// Execute a script for its return code
    Exec {
        /// Program to execute
        command: String,

        /// Arguments to program
        args: Vec<String>,
    },
}

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[clap(subcommand)]
    command: CliSubcommand,
}

fn main() -> NunjackResult<CheckStatus> {
    let cli_args = CliArgs::parse();

    match cli_args.command {
        CliSubcommand::Exec { command, args } => Ok(ExecCheck::new(command, args).healthcheck()),
        CliSubcommand::Pid { pid } => Ok(PidCheck::from(pid).healthcheck()),
        CliSubcommand::PidFile { path } => Ok(PidCheck::try_from(&path)?.healthcheck()),
        CliSubcommand::Socket { sock } => Ok(SocketCheck::try_from(sock.as_str())?.healthcheck()),
    }
}
