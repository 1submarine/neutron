use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use serde_yaml::Error;

use crate::{
    astronomical::{Constellation, Galaxy, GalaxyBuilder},
    ident::Ident,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub id: Ident,
    galaxy: Galaxy,
}
impl World {
    pub fn builder(rng: &mut WyRand) -> WorldBuilder {
        WorldBuilder::new(rng)
    }
    pub fn save(&self) -> Result<String, serde_yaml::Error> {
        let ret: Result<String, Error> = serde_yaml::to_string(&self);
        if let Ok(ret) = ret {
            return Ok(ret);
        }
        Err(ret.unwrap_err())
    }
    pub fn load(data: String) -> Result<Self, Error> {
        let ret: Result<Self, Error> = serde_yaml::from_str(&data);
        if let Ok(ret) = ret {
            return Ok(ret);
        }
        Err(ret.unwrap_err())
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldBuilder {
    id: Ident,
    galaxyb: GalaxyBuilder,
}
impl WorldBuilder {
    pub fn new(rng: &mut WyRand) -> Self {
        Self {
            id: Ident::new(rng.generate::<u64>().to_string()),
            galaxyb: GalaxyBuilder::total(rng),
        }
    }
    pub fn constellations(mut self, rng: &mut WyRand, num: u8) -> Self {
        for _ in 0..num {
            self.galaxyb.add_constellation(rng);
        }
        self
    }
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.id = Ident::new(name);
        self
    }
    pub fn build(self) -> World {
        World {
            id: self.id.clone(),
            galaxy: self.galaxyb.build(),
        }
    }
}
