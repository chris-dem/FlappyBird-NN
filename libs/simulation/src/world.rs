use crate::*;
use rand::RngCore;
#[derive(Debug)]
pub struct World {
    crate birds : Vec<Bird>
}


impl World {
    crate fn random(rng : &mut dyn RngCore) -> Self {
        let birds = (0..40)
            .map(|_| Bird::random(rng))
            .collect();
        
        Self {
            birds
        }
    }

    pub fn birds(&self) -> &[Bird] {
        &self.birds
    }
}