use crate::argument_types::cluster_update::LogPrefix;
use crate::{auto_update, status, update};
use structopt::clap::arg_enum;
use structopt::StructOpt;
use failure::Error;

arg_enum! {
    #[derive(Debug)]
    pub enum LogLevel {
        DEBUG,
        INFO,
        WARNING,
        ERROR,
        CRITICAL
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CommandLine {
    /// Set the logging level
    #[structopt(long, default_value = "DEBUG")]
    log_level: LogLevel,

    /// Include timestamps in the logging output
    #[structopt(long, default_value = "")]
    log_prefix: LogPrefix,

    /// Include timestamps in the logging output
    #[structopt(long)]
    log_timestamps: bool,

    /// Use azure.json to set up state (including subscription, resource group, and azure access)
    #[structopt(long)]
    in_cluster: bool,

    #[structopt(subcommand)]
    cmd: Subcommand,
}
#[derive(Debug, StructOpt)]
pub enum Subcommand {
    AutoUpdate(auto_update::CmdAutoUpdate),
    Status(status::CmdStatus),
    Update(update::CmdUpdate),
}

impl CommandLine {
    pub fn run(self) -> Result<(), Error> {
        match self.cmd {
            Subcommand::AutoUpdate(auto_update) => auto_update.run(),
            Subcommand::Status(status) => status.run(),
            Subcommand::Update(update) => update.run(),
        }
    }
}