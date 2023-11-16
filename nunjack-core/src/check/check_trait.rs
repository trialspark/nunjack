use std::process::{ExitStatus, Termination};

pub enum CheckStatus {
    Success,
    Failure(u8),
}

impl CheckStatus {
    pub const IO_ERR: Self = Self::Failure(10);
    pub const NO_PERMISSION: Self = Self::Failure(77);
    pub const UNAVAILABLE: Self = Self::Failure(69);
    pub const USER_ERR: Self = Self::Failure(65);
    pub const UNKNOWN: Self = Self::Failure(255);
}

impl From<i32> for CheckStatus {
    fn from(value: i32) -> Self {
        if value == 0 {
            Self::Success
        } else {
            Self::Failure(value.unsigned_abs() as u8)
        }
    }
}

impl From<ExitStatus> for CheckStatus {
    fn from(value: ExitStatus) -> Self {
        value
            .code()
            .map(CheckStatus::from)
            .unwrap_or(CheckStatus::UNKNOWN)
    }
}

impl From<std::io::Error> for CheckStatus {
    fn from(value: std::io::Error) -> Self {
        use std::io::ErrorKind::*;

        match value.kind() {
            NotFound => Self::UNAVAILABLE,
            PermissionDenied => Self::NO_PERMISSION,
            _ => Self::UNKNOWN,
        }
    }
}

pub trait Check {
    fn healthcheck(&self) -> CheckStatus;
}

impl Termination for CheckStatus {
    fn report(self) -> std::process::ExitCode {
        match self {
            Self::Success => std::process::ExitCode::SUCCESS,
            Self::Failure(code) => std::process::ExitCode::from(code),
        }
    }
}
