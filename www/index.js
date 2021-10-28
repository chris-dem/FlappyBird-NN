import * as sim from "lib-simulation-wasm";
import p5 from "p5";
import SimRenderer from "./apps/myrender";

const WINDOW_WIDTH = 1536;
const WINDOW_HEIGHT = 726;
const sketch = (p) => {
    let simulation;
    let renderer;
    let canvas;
    p.setup = () => {
      canvas = p.createCanvas(WINDOW_WIDTH , WINDOW_HEIGHT);
      simulation = new sim.Simulation(p.width, p.height);;
      renderer = new SimRenderer(simulation);
    };
  
    p.draw = () => {
        p.background(51);
        let text = simulation.step()
        if(text) {
          renderer.addStatistic(text);
        }
        renderer.show(p)
        
    };

    
};
  
new p5(sketch);
