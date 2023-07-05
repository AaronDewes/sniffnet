//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use crate::{IpVersion, Language, TransProtocol};
use std::collections::HashSet;
use crate::translations::translations_3::{interval_translation, single_translation, well_known_translation};

/// Possible filters applicable to network traffic
#[derive(Clone)]
pub struct Filters {
    /// Internet Protocol version
    pub ip: IpVersion,
    /// Transport layer protocol
    pub transport: TransProtocol,
    /// Transport ports
    pub ports: PortFilter,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            ip: IpVersion::Other,
            transport: TransProtocol::Other,
            ports: PortFilter::Interval([0,65535]),
        }
    }
}

/// Enum representing the possible kinds of port filter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(clippy::enum_variant_names)]
pub enum PortFilter {
    Interval([u16;2]),
    Single(u16),
    WellKnown(HashSet<u16>),
}

impl PortFilter {
    pub fn all_strings(language: Language) -> Vec<&'static str> {
        vec![
            interval_translation(language),
            single_translation(language),
            well_known_translation(language),
        ]
    }

    pub fn get_picklist_label(self, language: Language) -> &'static str {
        match self {
            PortFilter::Interval(_) => interval_translation(language),
            PortFilter::Single(_) => single_translation(language),
            PortFilter::WellKnown(_) => well_known_translation(language),
        }
    }
}
