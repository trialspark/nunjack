use std::process;

use crate::{Check, CheckStatus};

pub struct ExecCheck {
    command: String,
    args: Vec<String>,
}

impl ExecCheck {
    pub fn new(command: String, args: Vec<String>) -> Self {
        ExecCheck { command, args }
    }
}

impl Check for ExecCheck {
    fn healthcheck(&self) -> CheckStatus {
        match process::Command::new(&self.command)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .args(&self.args)
            .output()
        {
            Ok(process::Output { status, .. }) => status.into(),
            Err(err) => err.into(),
        }
    }
}
