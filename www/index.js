import * as sim from "lib-simulation-wasm";
import p5 from "p5";
import Bird from './apps/bird';
// import Pipe from './apps/pipe';
import PipeHandler from './apps/pipehandler';




const sketch = (p) => {
    let canvas;
    let logo;
    let logoWidth = 250;
    let logoHeight = 114;

    let simulation;
    let world
    let bird;
    let pipe;
    let state;
    let font;
    p.preload = () => {
      font = p.loadFont('./assets/ZenKurenaido-Regular.ttf')
    }
    p.setup = () => {
      canvas = p.createCanvas(p.windowWidth, p.windowHeight);
      simulation = new sim.Simulation();
      world = simulation.world();
      bird = new Bird(p.width / 3,p.height/ 2);
      pipe = new PipeHandler(p,bird);
    };
  
    p.draw = () => {
        p.background(51);
        pipe.update();
        bird.update(p);
        if(bird.state)
          pipe.checkCollision(bird)
        
        bird.show(p)
        pipe.show();
        p.push()
        p.fill('#0');
        p.textFont(font);
        p.textSize(30);
        p.text(bird.counter,p.width /2 ,p.height / 8)
        p.pop()
        
        p.line(p.width / 3, 0,p.width/3,p.height);
        p.line(bird.pos.x, 0,bird.pos.x,p.height);
        p.line(0, bird.pos.y,p.width,bird.pos.y);

        
    };
  
    p.windowResized = () => {
      p.resizeCanvas(p.windowWidth, p.windowHeight);
    };
    
    p.keyPressed = () => {
      if(p.keyCode == 32 && bird.state)
          bird.flap();
        if(p.keyCode == p.LEFT_ARROW)
          pause = !pause;
    }
  };
  
  new p5(sketch);