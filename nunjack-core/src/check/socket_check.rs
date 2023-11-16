use crate::{Check, CheckStatus};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

pub struct SocketCheck(SocketAddr);

impl Check for SocketCheck {
    fn healthcheck(&self) -> CheckStatus {
        match TcpStream::connect_timeout(&self.0, std::time::Duration::from_secs(10)) {
            Ok(_) => CheckStatus::Success,
            _ => CheckStatus::IO_ERR,
        }
    }
}

impl From<SocketAddr> for SocketCheck {
    fn from(value: SocketAddr) -> Self {
        SocketCheck(value)
    }
}

impl TryFrom<&str> for SocketCheck {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut addrs = value.to_socket_addrs()?;
        Ok(addrs
            .next()
            .map(SocketCheck::from)
            .ok_or(std::io::ErrorKind::NotFound)?)
    }
}
