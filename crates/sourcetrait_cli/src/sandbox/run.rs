use crate::sandbox::*;

pub(crate) fn run_cmd(cmd: BoxCommand) -> anyhow::Result<()> {
    match cmd.command {
        BoxSubcommand::Pull(cmd) => run_pull(cmd),
        BoxSubcommand::Ssh(cmd) => run_ssh(cmd),
        BoxSubcommand::Running(cmd) => run_running(cmd),
        BoxSubcommand::Start(_cmd) => todo!(),
        BoxSubcommand::Stop(_cmd) => todo!(),
        BoxSubcommand::Refresh(_cmd) => todo!(),
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
        name: cmd.name
    };
    
    lib_box::ssh(opts)
        .map_err(|e| anyhow::anyhow!("Failed to shell into image :: {e}"))
}

pub(crate) fn run_running(cmd: RunningCommand) -> anyhow::Result<()> {
    let opts = match cmd.name.as_str() {
        RunningCommand::MACHINE => lib_box::IsRunningOptions::Machine,
        _ => lib_box::IsRunningOptions::Container {
            name: cmd.name,
        },
    };
    
    match lib_box::is_running(opts) {
        Ok(is_running) => {
            println!("{is_running}");
            Ok(())
        },
        Err(e) => Err(anyhow::anyhow!("Failed to inspect podman :: {e}")),
    }
}