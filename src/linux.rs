#![allow(dead_code)]

use crate::MacAddressError;
use nix::{ifaddrs::*, sys::socket::SockAddr};

/// Uses the `getifaddrs` call to retrieve a list of network interfaces on the
/// host device and returns the first MAC address listed that isn't
/// local-loopback or if a name was specified, that name.
pub fn get_mac(name: Option<&str>) -> Result<Vec<[u8; 6]>, MacAddressError> {
    let ifiter = getifaddrs()?;

    let mut rts = Vec::new();
    for interface in ifiter {
        if let Some(SockAddr::Link(link)) = interface.address {
            let bytes = link.addr();

            if let Some(name) = name {
                if interface.interface_name == name {
                    // return Ok(vec![bytes]);
                    rts.push(bytes);
                }
            } else if bytes.iter().any(|&x| x != 0) {
                rts.push(bytes);
                // return Ok(Some(bytes));
            }
        }
    }

    Ok(rts)
}

pub fn get_ifname(mac: &[u8; 6]) -> Result<Option<String>, MacAddressError> {
    let ifiter = getifaddrs()?;

    for interface in ifiter {
        if let Some(SockAddr::Link(link)) = interface.address {
            let bytes = link.addr();

            if &bytes == mac {
                return Ok(Some(interface.interface_name));
            }
        }
    }

    Ok(None)
}
