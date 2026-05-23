//use crate::*;

/// Sandboxed command-line environments for data analytics scripting
#[derive(Debug, clap::Parser)]
pub(crate) struct BoxCommand {
    #[clap(subcommand)]
    pub(crate) command: BoxSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum BoxSubcommand {
    Pull(PullCommand),
    #[clap(alias = "sh")]
    Shell(ShellCommand),
    Start(StartCommand),
    Stop(StopCommand),
    #[clap(alias = "re")]
    Refresh(RefreshCommand),
}

/// Downloads a sandbox image
#[derive(Debug, clap::Parser)]
pub(crate) struct PullCommand {
    /// Image URL or filepath 
    pub(crate) source: String,
    /// Container name, non-default
    pub(crate) name: Option<String>,
}

/// Shells into a container, starting it if necessary
#[derive(Debug, clap::Parser)]
pub(crate) struct ShellCommand {
    /// Container name
    pub(crate) name: String,
}

/// Starts a container 
#[derive(Debug, clap::Parser)]
pub(crate) struct StartCommand {
    /// Container name
    pub(crate) name: String,
}

/// Stops a container 
#[derive(Debug, clap::Parser)]
pub(crate) struct StopCommand {
    /// Container name
    pub(crate) name: String,
}

/// Updates an image and restarts it if running
#[derive(Debug, clap::Parser)]
pub(crate) struct RefreshCommand {
    /// Container name
    pub(crate) name: String,
}
