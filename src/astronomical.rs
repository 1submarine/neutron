use crate::{ident::Ident, name};
use nalgebra::geometry::Point2;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use std::cmp::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}
impl Point {
    fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
    fn dissolve(&self) -> (i8, i8) {
        (self.x, self.y)
    }
    fn nalgebra(&self) -> Point2<f64> {
        Point2::new(self.x as f64, self.y as f64)
    }
}
type CoordinateMap<T> = HashMap<Point, T>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Galaxy {
    id: Ident,
    pub constellations: CoordinateMap<Constellation>,
    pub connections: Vec<(Point, Point)>,
}
impl Galaxy {
    fn builder() -> GalaxyBuilder {
        GalaxyBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalaxyBuilder {
    id: Ident,
    constellations: Vec<ConstellationBuilder>,
}
impl GalaxyBuilder {
    pub fn total(rng: &mut WyRand) -> Self {
        let mut ret = Self::new();

        for _ in 0..rng.generate_range(4..8) {
            ret.constellations.push(ConstellationBuilder::total(rng));
        }

        ret
    }
    pub fn new() -> Self {
        Self {
            id: Ident::new(name::galaxy()),
            constellations: Vec::new(),
        }
    }
    pub fn add_constellation(&mut self) {
        self.constellations.push(Constellation::builder());
    }
    pub fn build(mut self, rng: &mut WyRand) -> Galaxy {
        let constellations = {
            let mut ret = HashMap::new();
            for con in self.constellations.drain(..) {
                ret.insert(
                    Point::new(rng.generate::<i8>(), rng.generate::<i8>()),
                    con.build(rng),
                );
            }
            ret
        };
        let connections = {
            let mut ret: Vec<(Point, Point)> = Vec::new();

            for (&cpos, ccons) in constellations.iter() {
                for (&tpos, tcons) in constellations.iter() {
                    /*
                    If the connection does not exist
                    AND the points are not the same
                    AND the point is in a radius of 32
                    */
                    if !(ret.contains(&(cpos, tpos))
                        || ret.contains(&(tpos, cpos))
                        || tcons == ccons)
                        && nalgebra::distance(&cpos.nalgebra(), &tpos.nalgebra()) <= 128.0
                    {
                        // then add a connection
                        ret.push((cpos, tpos));
                    }
                }
            }

            ret
        };

        Galaxy {
            id: self.id,
            constellations,
            connections,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Constellation {
    id: Ident,
    pub systems: HashMap<Point, System>,
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

        for _ in 0..rng.generate_range(4..8) {
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
    pub fn build(mut self, rng: &mut WyRand) -> Constellation {
        Constellation {
            id: self.id,
            systems: {
                let mut ret = HashMap::new();
                for i in self.systems.drain(..) {
                    ret.insert(
                        Point::new(rng.generate::<i8>(), rng.generate::<i8>()),
                        i.build(),
                    );
                }
                ret
            },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

        for _ in 0..rng.generate_range(4..12u8) {
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
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

fn tolerance(one: i8, two: i8, tolerance: u8) -> bool {
    one + two <= tolerance as i8
}
