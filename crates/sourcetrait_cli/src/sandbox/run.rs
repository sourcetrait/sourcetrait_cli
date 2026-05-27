use crate::sandbox::*;

pub(crate) fn run_cmd(cmd: BoxCmd) -> anyhow::Result<()> {
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

pub(crate) fn run_pull(cmd: PullCommand) -> anyhow::Result<()> {
    let opts = lib_box::PullOptions {
        source: cmd.source,
    };
    
    lib_box::pull(opts)
        .map_err(|e| anyhow::anyhow!("Failed to pull image :: {e}"))
}

pub(crate) fn run_ssh(cmd: SshCommand) -> anyhow::Result<()> {
    let opts = lib_box::SshOptions {
        container: cmd.container
    };
    
    lib_box::ssh(opts)
        .map_err(|e| anyhow::anyhow!("Failed to shell into image :: {e}"))
}

pub(crate) fn run_start(cmd: StartCommand) -> anyhow::Result<()> {
    let opts = lib_box::StartOptions { name: cmd.name };
    lib_box::start(opts)
        .map_err(|e| anyhow::anyhow!("Failed to start container :: {e}"))
}

pub(crate) fn run_running(cmd: RunningCommand) -> anyhow::Result<()> {
    let opts = lib_box::IsRunningOptions::Container { name: cmd.name };
    match lib_box::is_running(opts) {
        Ok(is_running) => {
            println!("{is_running}");
            Ok(())
        },
        Err(e) => Err(anyhow::anyhow!("Failed to inspect podman :: {e}")),
    }
}

pub(crate) fn run_machine_running() -> anyhow::Result<()> {
    match lib_box::is_running(lib_box::IsRunningOptions::Machine) {
        Ok(is_running) => {
            println!("{is_running}");
            Ok(())
        },
        Err(e) => Err(anyhow::anyhow!("Failed to inspect podman :: {e}")),
    }
}