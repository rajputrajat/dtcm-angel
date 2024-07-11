use std::net::{AddrParseError, IpAddr};
use tokio::process::Command;

use crate::Result;

/// Returns the public ip address
/// Error: multiple cases
pub async fn public_ip() -> Result<IpAddr> {
    String::from_utf8(
        Command::new("curl")
            .arg("ifconfig.me/ip")
            .output()
            .await?
            .stdout,
    )?
    .parse()
    .map_err(|e: AddrParseError| {
        error!("Failed to get public IP: {:?}", e);
        e.into()
    })
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn public_ip_works() {
        assert!(super::public_ip().await.is_ok())
    }
}
