#![allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]

use crate::Language;

pub fn ports_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Ports",
        Language::IT => "Porte",
        _ => "Ports",
    }
}

pub fn port_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Port",
        Language::IT => "Porta",
        _ => "Port",
    }
}

pub fn interval_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Interval",
        Language::IT => "Intervallo",
        _ => "Interval",
    }
}

pub fn single_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Single",
        Language::IT => "Singola",
        _ => "Single",
    }
}

pub fn well_known_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Well known",
        Language::IT => "Note",
        _ => "Well known",
    }
}
