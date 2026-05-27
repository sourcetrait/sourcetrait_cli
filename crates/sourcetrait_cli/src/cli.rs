use crate::*;

#[derive(Debug, clap::Parser)]
#[clap(version,about)]
#[clap(styles = clapx::CLAP_STYLE_SOURCETRAIT)]
pub(crate) struct SrcTraitCli {
    #[clap(subcommand)]
    pub(crate) command: SrcTraitCommand,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum SrcTraitCommand {
    Note(note::cli::NoteCommand),
    Box(sandbox::cli::BoxCmd),
}
