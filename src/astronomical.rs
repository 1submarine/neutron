use crate::ident::Ident;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Galaxy {
    constellations: HashMap<(i8, i8), Constellation>,
}
impl Galaxy {
    fn builder() -> GalaxyBuilder {
        GalaxyBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalaxyBuilder {
    constellations: HashMap<(i8, i8), Constellation>,
}
impl GalaxyBuilder {
    pub fn new() -> Self {
        Self {
            constellations: HashMap::new(),
        }
    }
    pub fn constellation(&mut self, rng: &mut WyRand, cons: Constellation) {
        self.constellations
            .insert((rng.generate::<i8>(), rng.generate::<i8>()), cons);
    }
    pub fn build(self) -> Galaxy {
        Galaxy {
            constellations: self.constellations,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Constellation {
    systems: Vec<System>,
}
impl Constellation {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct System {
    id: Ident,
    planets: Vec<Planet>,
}
impl System {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            id: Ident::new(name),
            planets: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Planet {
    id: Ident,
}
