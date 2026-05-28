use crate::*;

pub(crate) type CliResult<T, E = CliError> = Result<T,E>;

#[derive(Debug, snafu::Snafu)]
pub enum CliError {
    #[snafu(transparent)]
    LibSandbox { source: lib_sandbox::SandboxError },
    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync>>
    },
}
