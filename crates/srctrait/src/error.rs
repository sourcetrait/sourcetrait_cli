#[derive(Debug, snafu::Snafu)]
pub enum SrcTraitError {
    IO { message: String, source: std::io::Error },
}

pub type SrcTraitResult<T> = std::result::Result<T, SrcTraitError>;
