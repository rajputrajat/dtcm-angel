use crate::{Result, UtilsError as Error};

/// Returns the mac address for the system
pub fn mac_addr() -> Result<mac_address::MacAddress> {
    mac_address::get_mac_address()?.ok_or_else(|| {
        error!("none received as mac-address");
        Error::MacAddressNone
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn mac_addr_works() {
        assert!(super::mac_addr().is_ok());
    }
}
