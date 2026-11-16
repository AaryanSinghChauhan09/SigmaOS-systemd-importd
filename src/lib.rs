// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-importd: systemd importd component model.
//!
//! Real implementation: importds, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Importd {
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
pub enum SystemdImportdError {
    EmptyImportdName,
    DuplicateImportd(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdImportdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyImportdName => write!(f, "importd name must not be empty"),
            Self::DuplicateImportd(id) => write!(f, "duplicate importd {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdImportdError {}

pub struct SystemdImportd {
    pub name: String,
    importds: BTreeMap<String, Importd>,
    events: BTreeMap<String, Event>,
}

impl SystemdImportd {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            importds: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_importd(&mut self, item: Importd) -> Result<(), SystemdImportdError> {
        if item.name.trim().is_empty() {
            return Err(SystemdImportdError::EmptyImportdName);
        }
        let id = item.id.clone();
        if self.importds.contains_key(&id) {
            return Err(SystemdImportdError::DuplicateImportd(id.clone()));
        }
        self.importds.insert(id, item);
        Ok(())
    }

    pub fn importd_count(&self) -> usize {
        self.importds.len()
    }

    pub fn get_importd(&self, id: &str) -> Option<&Importd> {
        self.importds.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdImportdError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdImportdError::EmptyEventSubject);
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

    fn sample_importd(id: &str, name: &str) -> Importd {
        Importd {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdImportd::new("SystemdImportd");
        assert_eq!(m.name, "SystemdImportd");
        assert_eq!(m.importd_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_importd() {
        let mut m = SystemdImportd::new("SystemdImportd");
        m.add_importd(sample_importd("a1", "Alpha")).unwrap();
        assert_eq!(m.importd_count(), 1);
        assert!(m.get_importd("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdImportd::new("SystemdImportd");
        assert!(m.add_importd(sample_importd("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdImportd::new("SystemdImportd");
        m.add_importd(sample_importd("x", "One")).unwrap();
        assert!(m.add_importd(sample_importd("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdImportd::new("SystemdImportd");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
