use crate::{ident::Ident, name};
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Galaxy {
    id: Ident,
    constellations: HashMap<(i8, i8), Constellation>,
}
impl Galaxy {
    fn builder() -> GalaxyBuilder {
        GalaxyBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalaxyBuilder {
    id: Ident,
    constellations: HashMap<(i8, i8), ConstellationBuilder>,
}
impl GalaxyBuilder {
    pub fn total(rng: &mut WyRand) -> Self {
        let mut ret = Self::new();

        for _ in 0..rng.generate_range(1..8) {
            ret.constellations.insert(
                (rng.generate::<i8>(), rng.generate::<i8>()),
                ConstellationBuilder::total(rng),
            );
        }

        ret
    }
    pub fn new() -> Self {
        Self {
            id: Ident::new(name::galaxy()),
            constellations: HashMap::new(),
        }
    }
    pub fn add_constellation(&mut self, rng: &mut WyRand) {
        self.constellations.insert(
            (rng.generate::<i8>(), rng.generate::<i8>()),
            Constellation::builder(),
        );
    }
    pub fn build(mut self) -> Galaxy {
        Galaxy {
            id: self.id,
            constellations: {
                let mut ret = HashMap::new();
                self.constellations.drain().for_each(|(loc, con)| {
                    ret.insert(loc, con.build());
                });
                ret
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Constellation {
    id: Ident,
    systems: Vec<System>,
}
impl Constellation {
    pub fn builder() -> ConstellationBuilder {
        ConstellationBuilder::new()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstellationBuilder {
    id: Ident,
    systems: Vec<SystemBuilder>,
}
impl ConstellationBuilder {
    pub fn total(rng: &mut WyRand) -> Self {
        let mut ret = Self::new();

        for _ in 0..rng.generate_range(1..8) {
            ret.systems.push(SystemBuilder::total(rng))
        }

        ret
    }
    pub fn new() -> Self {
        Self {
            id: Ident::new(name::constellation()),
            systems: Vec::new(),
        }
    }
    pub fn add_system(mut self) -> Self {
        self.systems.push(System::builder());
        self
    }
    pub fn build(mut self) -> Constellation {
        Constellation {
            id: self.id,
            systems: {
                let mut ret = Vec::new();
                self.systems.drain(..).for_each(|i| {
                    ret.push(i.build());
                });
                ret
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct System {
    id: Ident,
    planets: Vec<Planet>,
}
impl System {
    pub fn builder() -> SystemBuilder {
        SystemBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemBuilder {
    id: Ident,
    planets: Vec<PlanetBuilder>,
}
impl SystemBuilder {
    pub fn total(rng: &mut WyRand) -> Self {
        let mut ret = Self::new();

        for _ in 0..rng.generate_range(1..8) {
            ret.planets.push(PlanetBuilder::total(rng))
        }

        ret
    }
    pub fn new() -> Self {
        Self {
            id: Ident::new(name::system()),
            planets: Vec::new(),
        }
    }
    pub fn build(mut self) -> System {
        System {
            id: self.id,
            planets: {
                let mut ret = Vec::new();
                for i in self.planets.drain(..) {
                    ret.push(i.build());
                }
                ret
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Planet {
    id: Ident,
}
impl Planet {
    pub fn builder() -> PlanetBuilder {
        PlanetBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanetBuilder {
    id: Ident,
}

impl PlanetBuilder {
    pub fn new() -> Self {
        Self {
            id: Ident::new(name::planet()),
        }
    }
    pub fn build(self) -> Planet {
        Planet { id: self.id }
    }
    pub fn total(rng: &mut WyRand) -> Self {
        let mut ret = Self::new();
        ret
    }
}
