use crate::*;

pub fn run() -> ExitCode {
    match run_cli() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            let source = e.source()
                .map_or(String::new(), |s| format!("\n       {s}"));

            eprintln!("{STYL_ERROR}error:{STYL_ERROR:#} {e}{source}");
            ExitCode::FAILURE
        }
    }
}

fn run_cli() -> anyhow::Result<()> {
    let cli = SrcTraitCli::parse();
    match cli.command {
        SrcTraitCommand::Note(note_cmd) => note::run::run_cmd(note_cmd),
    }
}
