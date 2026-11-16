// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-app-indicator: app indicator component model.
//!
//! Real implementation: indicators, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indicator {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppIndicatorError {
    EmptyIndicatorName,
    DuplicateIndicator(String),
    EmptyEventSubject,
}

impl fmt::Display for AppIndicatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIndicatorName => write!(f, "indicator name must not be empty"),
            Self::DuplicateIndicator(id) => write!(f, "duplicate indicator {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AppIndicatorError {}

pub struct AppIndicator {
    pub name: String,
    indicators: BTreeMap<String, Indicator>,
    events: BTreeMap<String, Event>,
}

impl AppIndicator {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            indicators: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_indicator(&mut self, item: Indicator) -> Result<(), AppIndicatorError> {
        if item.name.trim().is_empty() {
            return Err(AppIndicatorError::EmptyIndicatorName);
        }
        let id = item.id.clone();
        if self.indicators.contains_key(&id) {
            return Err(AppIndicatorError::DuplicateIndicator(id.clone()));
        }
        self.indicators.insert(id, item);
        Ok(())
    }

    pub fn indicator_count(&self) -> usize {
        self.indicators.len()
    }

    pub fn get_indicator(&self, id: &str) -> Option<&Indicator> {
        self.indicators.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AppIndicatorError> {
        if event.subject.trim().is_empty() {
            return Err(AppIndicatorError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_indicator(id: &str, name: &str) -> Indicator {
        Indicator {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AppIndicator::new("AppIndicator");
        assert_eq!(m.name, "AppIndicator");
        assert_eq!(m.indicator_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_indicator() {
        let mut m = AppIndicator::new("AppIndicator");
        m.add_indicator(sample_indicator("a1", "Alpha")).unwrap();
        assert_eq!(m.indicator_count(), 1);
        assert!(m.get_indicator("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AppIndicator::new("AppIndicator");
        assert!(m.add_indicator(sample_indicator("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AppIndicator::new("AppIndicator");
        m.add_indicator(sample_indicator("x", "One")).unwrap();
        assert!(m.add_indicator(sample_indicator("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AppIndicator::new("AppIndicator");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
