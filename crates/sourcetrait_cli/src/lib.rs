pub(crate) mod cli;
pub(crate) mod error;
pub(crate) mod note;
pub(crate) mod run;
pub(crate) mod sandbox;

pub use run::run;

pub(crate) use self::{
    cli::*,
    error::*,
};

pub(crate) use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};

pub(crate) use snafu::Error;
pub(crate) use clap::Parser;
pub(crate) use sourcetrait_lib_note as lib;
pub(crate) use sourcetrait_lib_sandbox as lib_sandbox;
pub(crate) use sourcetrait_clapx::{self as clapx, styl::srctrait::*, subcmd::cli::CliCommand};
pub(crate) use sourcetrait_tomlx::{self as tomlx, starter::trim_starter_toml_file_comments, FromToml, ToStarterToml};
