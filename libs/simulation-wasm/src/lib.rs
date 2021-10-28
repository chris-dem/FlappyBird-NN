use rand::{prelude::ThreadRng, thread_rng};
use wasm_bindgen::{prelude::*};
use serde::Serialize;
use lib_simulation as sim;
mod utils;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s :&str);
}

#[wasm_bindgen]
pub struct Simulation {
    sim : sim::Simulation,
    rng : ThreadRng
}

#[derive(Clone,Debug,Serialize)]
pub struct World {
    pipes : Vec<Pipe>,
    birds: Vec<Bird>
}

#[derive(Clone,Debug,Serialize)]
pub struct Pipe{
    pub x : f32,
    pub y : f32,
}

#[derive(Clone,Debug,Serialize)]
pub struct Bird{
    pub x : f32,
    pub y : f32,
    pub state : bool,
    pub counter : usize
}


#[wasm_bindgen]
pub struct Statistics {
    pub avg : f32,
    pub max : f32,
    pub pop_num : usize
}

#[wasm_bindgen]
impl Statistics {
    pub fn avg(&self) -> f32 {
        self.avg
    }
    pub fn max(&self) -> f32 {
        self.max
    }
    pub fn pop_num(&self) -> usize {
        self.pop_num
    }
}

impl From<&sim::Statistics> for Statistics {
    fn from(stat : &sim::Statistics) -> Self {
        Self {
            avg : stat.ga.avg_fitness(),
            max : stat.ga.max_fitness(),
            pop_num : stat.generation,
        }
    }
}

impl From<&sim::World> for World {
    fn from(world : &sim::World) -> Self {
        let birds = world.birds.iter().map(Bird::from).collect();
        let pipes = world.pipe_handler.pipes.iter().map(Pipe::from).collect();
        Self {birds ,pipes  }
    }
}

impl From<&sim::Bird> for Bird {
    fn from(bird : &sim::Bird) -> Self {
        Self {
            x : bird.position().x,
            y : bird.position().y,
            state : bird.state(),
            counter : bird.counter()
        }
    }
}

impl From<&sim::Pipe> for Pipe {
    fn from(pipe : &sim::Pipe) -> Self {
        Self {
            x : pipe.position().x,
            y : pipe.position().y,
        }
    }
}





#[derive(Clone,Debug,Serialize)]
pub struct PipeStruct {
    loc_s : Vec<(f32,f32)>,

}
#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(x : f32, y : f32) -> Self {
        console_error_panic_hook::set_once();
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(x,y,&mut rng);
        Self {
            sim,
            rng
        }
    }

    pub fn step(&mut self) -> Option<Statistics>{
        self.sim.step(&mut self.rng).map(|stats| Statistics::from(&stats))
    }

    pub fn world(&self) -> JsValue {
        JsValue::from_serde(&World::from(self.sim.world())).unwrap()
    }
    
    pub fn current_gen(&self) -> usize {
        self.sim.pop_no
    }
}
