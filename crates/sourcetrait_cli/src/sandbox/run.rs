use snafu::ResultExt;

use crate::{*, sandbox::*};

pub(crate) fn run_cmd(cmd: BoxCmd) -> CliResult<()> {
    match cmd.command {
        BoxCommand::Pull(cmd) => run_pull(cmd),
        BoxCommand::Ssh(cmd) => run_ssh(cmd),
        BoxCommand::Running(cmd) => run_running(cmd),
        BoxCommand::Start(cmd) => run_start(cmd),
        BoxCommand::Stop(_cmd) => todo!(),
        BoxCommand::Refresh(_cmd) => todo!(),
        BoxCommand::Machine(subcmd) => match subcmd {
            MachineSubcommand::Running => run_machine_running(),
        },
    }
}

pub(crate) fn run_pull(cmd: PullCommand) -> CliResult<()> {
    let opts = lib_sandbox::PullOptions {
        source: cmd.source,
    };
    
    lib_sandbox::pull(opts)
        .with_whatever_context(|_| "Failed to pull image")
}

pub(crate) fn run_ssh(cmd: SshCommand) -> CliResult<()> {
    let opts = lib_sandbox::SshOptions {
        container: cmd.container
    };
    
    lib_sandbox::ssh(opts)
        .with_whatever_context(|_| "Failed to shell into image")
    
}

pub(crate) fn run_start(cmd: StartCommand) -> CliResult<()> {
    let opts = lib_sandbox::StartOptions { name: cmd.name };
    lib_sandbox::start(opts)
        .with_whatever_context(|_| "Failed to start container")
}

pub(crate) fn run_running(cmd: RunningCommand) -> CliResult<()> {
    let opts = lib_sandbox::IsRunningOptions::Container { name: cmd.name };
    match lib_sandbox::is_running(opts) {
        Ok(is_running) => {
            println!("{is_running}");
            Ok(())
        },
        Err(e) => snafu::whatever!("Failed to inspect podman :: {e}"),
    }
}

pub(crate) fn run_machine_running() -> CliResult<()> {
    match lib_sandbox::is_running(lib_sandbox::IsRunningOptions::Machine) {
        Ok(is_running) => {
            println!("{is_running}");
            Ok(())
        },
        Err(e) => snafu::whatever!("Failed to inspect podman :: {e}"),
    }
}