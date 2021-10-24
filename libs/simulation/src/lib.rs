
#![feature(crate_visibility_modifier)]
mod world;
mod bird;

pub use self::{world::*, bird::*};



use rand;
use serde::{Deserialize, Serialize};

pub struct Simulation {
    world : World
}


impl Simulation {
    pub fn random(rng : &mut dyn rand::RngCore) -> Self {
        Self {
            world :  World::random(rng)
        }
    }
    pub fn world(&self) -> &World {
        &self.world
    }
}


