use crate::sandbox::*;

pub(crate) fn run_cmd(cmd: BoxCommand) -> anyhow::Result<()> {
    match cmd.command {
        BoxSubcommand::Pull(cmd) => run_pull(cmd),
        BoxSubcommand::Shell(cmd) => run_shell(cmd),
        BoxSubcommand::Start(cmd) => todo!(),
        BoxSubcommand::Stop(cmd) => todo!(),
        BoxSubcommand::Refresh(cmd) => todo!(),
    }
}

pub(crate) fn run_pull(cmd: PullCommand) -> anyhow::Result<()> {
    let opts = lib_box::PullOptions {
        source: cmd.source,
    };
    
    lib_box::pull(opts)
        .map_err(|e| anyhow::anyhow!("Failed to pull image :: {e}"))
}

pub(crate) fn run_shell(cmd: ShellCommand) -> anyhow::Result<()> {
    let opts = lib_box::ShellOptions {
        name: cmd.name
    };
    
    lib_box::shell(opts)
        .map_err(|e| anyhow::anyhow!("Failed to shell into image :: {e}"))
}