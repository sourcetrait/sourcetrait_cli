//use crate::*;

/// Sandboxed command-line environments for data analytics scripting
#[derive(Debug, clap::Parser)]
pub(crate) struct BoxCmd {
    #[clap(subcommand)]
    pub(crate) command: BoxCommand,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum BoxCommand {
    Pull(PullCommand),
    Start(StartCommand),
    Ssh(SshCommand),
    Stop(StopCommand),
    Refresh(RefreshCommand),
    Running(RunningCommand),
    #[clap(subcommand)]
    Machine(MachineSubcommand),
}

/// Downloads a sandbox image
#[derive(Debug, clap::Parser)]
pub(crate) struct PullCommand {
    /// Image URL or filepath 
    pub(crate) source: String,
    /// Container name, non-default
    pub(crate) name: Option<String>,
}

/// Is a container running?
#[derive(Debug, clap::Parser)]
pub(crate) struct RunningCommand {
    /// Container name
    pub(crate) name: String,
}

/// SSHs into a container, starting it if necessary
#[derive(Debug, clap::Parser)]
pub(crate) struct SshCommand {
    /// Container name
    pub(crate) container: String,
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

/// Manages the podman machine
#[derive(Debug, clap::Subcommand)]
pub(crate) enum MachineSubcommand {
    Running,
}