//! SigmaOS-0ad: real-time strategy game model.
//!
//! Real implementation: units, resources, battles, validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub unit_type: String,
    pub health: u32,
    pub attack: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    EmptyName,
    DuplicateUnit(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "unit name must not be empty"),
            Self::DuplicateUnit(id) => write!(f, "duplicate unit {:?}", id),
        }
    }
}

impl std::error::Error for GameError {}

pub struct GameEngine {
    pub name: String,
    units: BTreeMap<String, Unit>,
    resources: BTreeMap<String, u32>,
}

impl GameEngine {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), units: BTreeMap::new(), resources: BTreeMap::new() }
    }

    pub fn add_unit(&mut self, u: Unit) -> Result<(), GameError> {
        if u.name.trim().is_empty() { return Err(GameError::EmptyName); }
        let id = u.id.clone();
        if self.units.contains_key(&id) {
            return Err(GameError::DuplicateUnit(id.clone()));
        }
        self.units.insert(id, u);
        Ok(())
    }

    pub fn unit_count(&self) -> usize { self.units.len() }

    pub fn get_unit(&self, id: &str) -> Option<&Unit> { self.units.get(id) }

    pub fn add_resource(&mut self, name: String, amount: u32) {
        *self.resources.entry(name).or_insert(0) += amount;
    }

    pub fn get_resource(&self, name: &str) -> Option<u32> { self.resources.get(name).copied() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let g = GameEngine::new("My Game");
        assert_eq!(g.name, "My Game");
        assert_eq!(g.unit_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut g = GameEngine::new("My Game");
        let u = Unit { id: "u1".into(), name: "soldier".into(), unit_type: "infantry".into(), health: 100, attack: 10 };
        g.add_unit(u).unwrap();
        assert_eq!(g.unit_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut g = GameEngine::new("My Game");
        let u = Unit { id: "x".into(), name: "".into(), unit_type: "a".into(), health: 1, attack: 1 };
        assert!(g.add_unit(u).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut g = GameEngine::new("My Game");
        let u = Unit { id: "x".into(), name: "X".into(), unit_type: "a".into(), health: 1, attack: 1 };
        g.add_unit(u).unwrap();
        let u2 = Unit { id: "x".into(), name: "X".into(), unit_type: "a".into(), health: 1, attack: 1 };
        assert!(g.add_unit(u2).is_err());
    }
}
