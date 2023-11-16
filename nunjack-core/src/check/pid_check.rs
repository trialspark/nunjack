use std::path::PathBuf;

use crate::{error::NunjackError, Check, CheckStatus};
use nix::{errno::Errno, libc, unistd};

pub struct PidCheck(unistd::Pid);

impl Check for PidCheck {
    fn healthcheck(&self) -> CheckStatus {
        match unistd::getpgid(Some(self.0)) {
            Ok(_pid) => CheckStatus::Success,
            Err(Errno::EPERM) => CheckStatus::NO_PERMISSION,
            Err(Errno::ESRCH) => CheckStatus::UNAVAILABLE,
            Err(Errno::EINVAL) => CheckStatus::USER_ERR,
            Err(_) => todo!(),
        }
    }
}

impl From<i32> for PidCheck {
    fn from(value: i32) -> Self {
        PidCheck(unistd::Pid::from_raw(value))
    }
}

impl TryFrom<&str> for PidCheck {
    type Error = NunjackError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pid_num = value.parse::<libc::pid_t>()?;
        Ok(PidCheck::from(pid_num))
    }
}

impl TryFrom<&PathBuf> for PidCheck {
    type Error = NunjackError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let source = std::fs::read_to_string(value)?;
        PidCheck::try_from(source.trim())
    }
}
