use rand::{prelude::ThreadRng, thread_rng};
use wasm_bindgen::prelude::*;
use serde::Serialize;
use lib_simulation as sim;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s :&str);
}

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim : sim::Simulation
}

#[derive(Clone,Debug,Serialize)]
pub struct World {
    pub birds : Vec<Bird>,
}


#[derive(Clone,Debug,Serialize)]
pub struct Bird{
    pub x : f32,
    pub y : f32,
}

impl From<&sim::World> for World {
    fn from(world : &sim::World) -> Self {
        let birds = world
            .birds()
            .iter()
            .map(Bird::from)
            .collect();
        Self {birds}
    }
}

impl From<&sim::Bird> for Bird {
    fn from(bird : &sim::Bird) -> Self {
        Self {
            x : bird.position().x,
            y : bird.position().y,
        }
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);
        Self {
            rng,
            sim
        }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }
}

#[cfg(test)]
mod tests {
    mod test_wasm {
        use crate::*;
        #[test]
        fn test_serde() {
            let sim = Simulation::new();
            println!("{:?}",sim.world());
        }
    }
}