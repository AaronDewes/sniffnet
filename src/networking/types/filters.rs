//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use crate::{IpVersion, TransProtocol};
use std::collections::HashSet;

/// Possible filters applicable to network traffic
#[derive(Clone)]
pub struct Filters {
    /// Internet Protocol version
    pub ip: IpVersion,
    /// Transport layer protocol
    pub transport: TransProtocol,
    /// Transport port numbers
    pub ports: HashSet<u16>,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            ip: IpVersion::Other,
            transport: TransProtocol::Other,
            ports: HashSet::default(),
        }
    }
}
