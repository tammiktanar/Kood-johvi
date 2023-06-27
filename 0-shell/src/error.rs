use crate::command::CmdExitCode;

/// Describes whether a given error is expected to happen as part of the command's usage
/// or is unexpected and needs developer attention
pub enum CmdError {
    Expected(CmdExitCode, anyhow::Error),
    Unexpected(anyhow::Error),
}

pub trait ExpectedError<T> {
    fn expected(self, exit_code: CmdExitCode) -> Result<T, CmdError>;
}

impl<T, E> ExpectedError<T> for Result<T, E>
    where E: Into<anyhow::Error>
{
    fn expected(self, exit_code: CmdExitCode) -> Result<T, CmdError> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(CmdError::Expected(exit_code, err.into())),
        }
    }
}

impl<E: Into<anyhow::Error>> From<E> for CmdError {
    fn from(err: E) -> Self {
        CmdError::Unexpected(err.into())
    }
}
