// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-system-service-4: system service 4 component model.
//!
//! Real implementation: services, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
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
pub enum SystemServiceFourError {
    EmptyServiceName,
    DuplicateService(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemServiceFourError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyServiceName => write!(f, "service name must not be empty"),
            Self::DuplicateService(id) => write!(f, "duplicate service {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemServiceFourError {}

pub struct SystemServiceFour {
    pub name: String,
    services: BTreeMap<String, Service>,
    events: BTreeMap<String, Event>,
}

impl SystemServiceFour {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            services: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_service(&mut self, item: Service) -> Result<(), SystemServiceFourError> {
        if item.name.trim().is_empty() {
            return Err(SystemServiceFourError::EmptyServiceName);
        }
        let id = item.id.clone();
        if self.services.contains_key(&id) {
            return Err(SystemServiceFourError::DuplicateService(id.clone()));
        }
        self.services.insert(id, item);
        Ok(())
    }

    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    pub fn get_service(&self, id: &str) -> Option<&Service> {
        self.services.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemServiceFourError> {
        if event.subject.trim().is_empty() {
            return Err(SystemServiceFourError::EmptyEventSubject);
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

    fn sample_service(id: &str, name: &str) -> Service {
        Service {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemServiceFour::new("SystemServiceFour");
        assert_eq!(m.name, "SystemServiceFour");
        assert_eq!(m.service_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_service() {
        let mut m = SystemServiceFour::new("SystemServiceFour");
        m.add_service(sample_service("a1", "Alpha")).unwrap();
        assert_eq!(m.service_count(), 1);
        assert!(m.get_service("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemServiceFour::new("SystemServiceFour");
        assert!(m.add_service(sample_service("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemServiceFour::new("SystemServiceFour");
        m.add_service(sample_service("x", "One")).unwrap();
        assert!(m.add_service(sample_service("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemServiceFour::new("SystemServiceFour");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
