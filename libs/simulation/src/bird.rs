use rand::{Rng, RngCore};

use nalgebra as na;

#[derive(Debug)]
pub struct Bird {
    crate position : na::Point2<f32>,
    crate rotation : na::Point2<f32>,
    crate speed : f32,
}


impl Bird {
    crate fn random(rng : &mut dyn RngCore) -> Self {
        Self {
            position : rng.gen(),
            rotation : rng.gen(),
            speed : rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    
    pub fn rotation(&self) -> na::Point2<f32> {
        self.rotation
    }

    
    pub fn speed(&self) -> f32 {
        self.speed
    }
}